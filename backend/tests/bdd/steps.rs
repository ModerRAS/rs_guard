use cucumber::{given, then, when, World};
use serde_json::json;
use std::path::Path;
use tokio::fs;
use super::RsGuardWorld;
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;
use backend::{config, metadata, app_router};
use std::sync::Arc;
use tokio::net::TcpListener;

/// 应用启动步骤
#[given(expr = "应用已启动")]
async fn app_started(world: &mut RsGuardWorld) {
    world.runtime().block_on(async {
        // 创建临时目录
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let temp_path = temp_dir.into_path();
        world.set_temp_dir(temp_path.clone());

        // 创建测试数据目录
        let test_data_dir = temp_path.join("test-data");
        let source_dir = test_data_dir.join("source");
        fs::create_dir_all(&source_dir).await.expect("Failed to create test data dir");
        world.set_test_data_dir(test_data_dir.clone());

        // 创建测试配置
        let app_config = config::AppConfig {
            watched_directories: vec![source_dir.to_string_lossy().to_string()],
            data_shards: 4,
            parity_shards: 2,
        };

        // 创建临时数据库
        let db_path = temp_path.join("test_db");
        let db = Arc::new(metadata::open_db(db_path.to_str().unwrap()).expect("Failed to open test DB"));

        // 创建应用状态
        let app_state = Arc::new(Mutex::new(AppStatus {
            watched_dirs: vec![source_dir.to_string_lossy().to_string()],
            data_shards: app_config.data_shards,
            parity_shards: app_config.parity_shards,
            ..Default::default()
        }));
        world.set_app_state(app_state.clone());

        // 启动服务器
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("Failed to bind random port");
        let addr = listener.local_addr().unwrap();
        world.set_server_address(addr);

        // 构建应用路由
        let app = app_router(app_state, db);

        // 在后台启动服务器
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        // 等待服务器启动
        sleep(Duration::from_millis(100)).await;
    });
}

/// 创建测试文件步骤
#[given(expr = "在监控目录中创建测试文件 {word}")]
async fn create_test_file(world: &mut RsGuardWorld, filename: String) {
    world.runtime().block_on(async {
        let test_data_dir = world.test_data_dir().clone();
        let source_dir = test_data_dir.join("source");
        let file_path = source_dir.join(filename);

        // 创建测试文件内容
        let content = format!("This is test file {} created at {}", 
            filename, chrono::Utc::now().to_rfc3339());
        
        fs::write(&file_path, content).await
            .unwrap_or_else(|_| panic!("Failed to create test file: {}", file_path.display()));
        
        // 等待文件监控处理
        sleep(Duration::from_millis(500)).await;
    });
}

/// 发送 HTTP 请求步骤
#[when(expr = "发送 {word} 请求到 {word}")]
async fn send_request(world: &mut RsGuardWorld, method: String, endpoint: String) {
    world.runtime().block_on(async {
        let client = Client::new();
        let base_url = format!("http://{}", world.server_address());
        let url = format!("{}{}", base_url, endpoint);

        let response = match method.as_str() {
            "GET" => client.get(&url).send().await,
            "POST" => client.post(&url).send().await,
            "PUT" => client.put(&url).send().await,
            "DELETE" => client.delete(&url).send().await,
            _ => panic!("Unsupported HTTP method: {}", method),
        };

        match response {
            Ok(resp) => {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                
                let json_response = json!({
                    "status": status.as_u16(),
                    "body": body
                });
                
                world.set_last_response(json_response);
            }
            Err(e) => {
                world.set_last_error(format!("Request failed: {}", e));
            }
        }
    });
}

/// 验证响应状态步骤
#[then(expr = "响应状态应该是 {int}")]
async fn verify_status(world: &mut RsGuardWorld, expected_status: u16) {
    let response = world.last_response()
        .expect("No response available");
    
    let actual_status = response["status"].as_u64()
        .expect("Status not found in response") as u16;
    
    assert_eq!(actual_status, expected_status, 
        "Expected status {}, got {}", expected_status, actual_status);
}

/// 验证响应包含特定字段步骤
#[then(expr = "响应应该包含字段 {word}")]
async fn verify_field_exists(world: &mut RsGuardWorld, field: String) {
    let response = world.last_response()
        .expect("No response available");
    
    let body = response["body"].as_str()
        .expect("Response body not found");
    
    let json_body: serde_json::Value = serde_json::from_str(body)
        .expect("Failed to parse response body as JSON");
    
    assert!(json_body.get(&field).is_some(), 
        "Response should contain field '{}'", field);
}

/// 验证响应字段值步骤
#[then(expr = "字段 {word} 应该是 {word}")]
async fn verify_field_value(world: &mut RsGuardWorld, field: String, expected_value: String) {
    let response = world.last_response()
        .expect("No response available");
    
    let body = response["body"].as_str()
        .expect("Response body not found");
    
    let json_body: serde_json::Value = serde_json::from_str(body)
        .expect("Failed to parse response body as JSON");
    
    let actual_value = json_body.get(&field)
        .expect(&format!("Field '{}' not found in response", field));
    
    let expected_json = if expected_value.parse::<i64>().is_ok() {
        serde_json::Value::Number(serde_json::Number::from(expected_value.parse::<i64>().unwrap()))
    } else if expected_value.parse::<f64>().is_ok() {
        serde_json::Value::Number(serde_json::Number::from_f64(expected_value.parse::<f64>().unwrap()).unwrap())
    } else if expected_value == "true" {
        serde_json::Value::Bool(true)
    } else if expected_value == "false" {
        serde_json::Value::Bool(false)
    } else {
        serde_json::Value::String(expected_value)
    };
    
    assert_eq!(actual_value, &expected_json,
        "Field '{}' should be '{}', got '{:?}'", field, expected_value, actual_value);
}

/// 验证文件存在步骤
#[then(expr = "文件 {word} 应该存在")]
async fn verify_file_exists(world: &mut RsGuardWorld, filename: String) {
    world.runtime().block_on(async {
        let test_data_dir = world.test_data_dir().clone();
        let file_path = test_data_dir.join("source").join(filename);
        
        assert!(file_path.exists(), 
            "File '{}' should exist at '{}'", filename, file_path.display());
    });
}

/// 验证文件内容步骤
#[then(expr = "文件 {word} 应该包含内容 {word}")]
async fn verify_file_content(world: &mut RsGuardWorld, filename: String, expected_content: String) {
    world.runtime().block_on(async {
        let test_data_dir = world.test_data_dir().clone();
        let file_path = test_data_dir.join("source").join(filename);
        
        let content = fs::read_to_string(&file_path).await
            .unwrap_or_else(|_| panic!("Failed to read file: {}", file_path.display()));
        
        assert!(content.contains(&expected_content),
            "File '{}' should contain '{}', got: '{}'", filename, expected_content, content);
    });
}

/// 等待步骤
#[when(expr = "等待 {int} 毫秒")]
async fn wait_ms(world: &mut RsGuardWorld, ms: u64) {
    world.runtime().block_on(async {
        sleep(Duration::from_millis(ms)).await;
    });
}

/// 清理步骤
#[then(expr = "清理测试环境")]
async fn cleanup(world: &mut RsGuardWorld) {
    world.cleanup().await;
}

/// 验证字段是数组类型
#[then(expr = "字段 {word} 应该是一个数组")]
async fn verify_field_is_array(world: &mut RsGuardWorld, field: String) {
    let response = world.last_response()
        .expect("No response available");
    
    let body = response["body"].as_str()
        .expect("Response body not found");
    
    let json_body: serde_json::Value = serde_json::from_str(body)
        .expect("Failed to parse response body as JSON");
    
    let field_value = json_body.get(&field)
        .expect(&format!("Field '{}' not found in response", field));
    
    assert!(field_value.is_array(), 
        "Field '{}' should be an array, got: {:?}", field, field_value);
}