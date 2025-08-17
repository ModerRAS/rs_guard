//! 用户验收测试 (UAT - User Acceptance Testing)
//! 
//! 这个模块包含用户验收测试用例，从用户的角度验证系统的功能性和可用性。
//! 这些测试模拟真实用户的使用场景，确保系统满足业务需求。

mod file_protection;
mod web_interface;
mod data_integrity;
mod configuration;
mod error_handling;
mod recovery;

pub use file_protection::*;
pub use web_interface::*;
pub use data_integrity::*;
pub use configuration::*;
pub use error_handling::*;
pub use recovery::*;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use backend::{config, metadata, app_router};
use shared::AppStatus;
use tempfile::tempdir;
use std::net::SocketAddr;
use tokio::net::TcpListener;

/// UAT 测试环境配置
#[derive(Debug)]
pub struct UatConfig {
    /// 测试数据目录
    pub test_data_dir: PathBuf,
    /// 监控目录
    pub watched_dirs: Vec<String>,
    /// 数据分片数
    pub data_shards: usize,
    /// 校验分片数
    pub parity_shards: usize,
    /// 服务器端口
    pub server_port: u16,
}

impl Default for UatConfig {
    fn default() -> Self {
        Self {
            test_data_dir: PathBuf::from("./test-data"),
            watched_dirs: vec!["./test-data/source".to_string()],
            data_shards: 4,
            parity_shards: 2,
            server_port: 0, // 0 表示随机端口
        }
    }
}

/// UAT 测试上下文
pub struct UatContext {
    /// 运行时
    runtime: Runtime,
    /// 配置
    config: UatConfig,
    /// 服务器地址
    server_address: SocketAddr,
    /// 临时目录
    temp_dir: PathBuf,
    /// 应用状态
    app_state: Arc<Mutex<AppStatus>>,
}

impl UatContext {
    pub async fn new(config: UatConfig) -> Self {
        let runtime = Runtime::new().expect("Failed to create runtime");
        
        // 创建临时目录
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let temp_path = temp_dir.into_path();
        
        // 创建测试数据目录结构
        let test_data_dir = temp_path.join("test-data");
        let source_dir = test_data_dir.join("source");
        tokio::fs::create_dir_all(&source_dir).await.expect("Failed to create test data dir");
        
        // 更新配置中的路径
        let mut config = config;
        config.test_data_dir = test_data_dir.clone();
        config.watched_dirs = vec![source_dir.to_string_lossy().to_string()];
        
        // 创建应用配置
        let app_config = config::AppConfig {
            watched_directories: config.watched_dirs.clone(),
            data_shards: config.data_shards,
            parity_shards: config.parity_shards,
        };
        
        // 创建临时数据库
        let db_path = temp_path.join("test_db");
        let db = Arc::new(metadata::open_db(db_path.to_str().unwrap()).expect("Failed to open test DB"));
        
        // 创建应用状态
        let app_state = Arc::new(Mutex::new(AppStatus {
            watched_dirs: config.watched_dirs.clone(),
            data_shards: app_config.data_shards,
            parity_shards: app_config.parity_shards,
            ..Default::default()
        }));
        
        // 启动服务器
        let listener = TcpListener::bind(&format!("127.0.0.1:{}", config.server_port))
            .await
            .expect("Failed to bind to port");
        let server_address = listener.local_addr().unwrap();
        
        // 构建应用路由
        let app = app_router(app_state.clone(), db);
        
        // 在后台启动服务器
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        
        // 等待服务器启动
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        Self {
            runtime,
            config,
            server_address,
            temp_dir,
            app_state,
        }
    }
    
    /// 获取服务器 URL
    pub fn server_url(&self) -> String {
        format!("http://{}", self.server_address)
    }
    
    /// 获取监控目录路径
    pub fn watched_dir(&self) -> PathBuf {
        self.temp_dir.join("test-data/source")
    }
    
    /// 获取运行时引用
    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }
    
    /// 获取应用状态
    pub fn app_state(&self) -> &Arc<Mutex<AppStatus>> {
        &self.app_state
    }
    
    /// 创建测试文件
    pub async fn create_test_file(&self, filename: &str, content: &str) -> PathBuf {
        let file_path = self.watched_dir().join(filename);
        tokio::fs::write(&file_path, content).await
            .expect("Failed to create test file");
        file_path
    }
    
    /// 检查文件是否存在
    pub async fn file_exists(&self, filename: &str) -> bool {
        let file_path = self.watched_dir().join(filename);
        tokio::fs::metadata(&file_path).await.is_ok()
    }
    
    /// 等待文件处理完成
    pub async fn wait_for_file_processing(&self, timeout_ms: u64) -> bool {
        let start = std::time::Instant::now();
        let timeout = tokio::time::Duration::from_millis(timeout_ms);
        
        while start.elapsed() < timeout {
            // 检查应用状态是否更新
            let state = self.app_state.lock().unwrap();
            if state.total_files > 0 {
                return true;
            }
            drop(state);
            
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        
        false
    }
    
    /// 清理测试环境
    pub async fn cleanup(self) {
        if self.temp_dir.exists() {
            tokio::fs::remove_dir_all(&self.temp_dir).await.ok();
        }
    }
}

/// UAT 测试断言
pub struct UatAssertions;

impl UatAssertions {
    /// 断言 HTTP 响应状态
    pub fn assert_status(response: &reqwest::Response, expected_status: u16) {
        assert_eq!(response.status().as_u16(), expected_status,
            "Expected status {}, got {}", expected_status, response.status().as_u16());
    }
    
    /// 断言 JSON 响应
    pub async fn assert_json(response: reqwest::Response) -> serde_json::Value {
        let text = response.text().await.expect("Failed to get response text");
        serde_json::from_str(&text).expect("Failed to parse JSON response")
    }
    
    /// 断言 JSON 字段存在
    pub fn assert_json_field(json: &serde_json::Value, field: &str) {
        assert!(json.get(field).is_some(), "Field '{}' not found in JSON response", field);
    }
    
    /// 断言 JSON 字段值
    pub fn assert_json_field_value(json: &serde_json::Value, field: &str, expected: &serde_json::Value) {
        let actual = json.get(field).expect(&format!("Field '{}' not found", field));
        assert_eq!(actual, expected, "Field '{}' value mismatch", field);
    }
    
    /// 断言文件存在
    pub async fn assert_file_exists(path: &std::path::Path) {
        assert!(tokio::fs::metadata(path).await.is_ok(), "File '{}' does not exist", path.display());
    }
    
    /// 断言文件内容
    pub async fn assert_file_content(path: &std::path::Path, expected_content: &str) {
        let content = tokio::fs::read_to_string(path).await.expect("Failed to read file");
        assert!(content.contains(expected_content), 
            "File '{}' should contain '{}', got: '{}'", 
            path.display(), expected_content, content);
    }
}

/// UAT 测试工具
pub struct UatUtils;

impl UatUtils {
    /// 生成随机文件内容
    pub fn generate_random_content(size: usize) -> String {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 "
            .chars().collect();
        
        (0..size)
            .map(|_| chars[rng.gen_range(0..chars.len())])
            .collect()
    }
    
    /// 生成随机文件名
    pub fn generate_random_filename() -> String {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        let prefix: String = (0..8)
            .map(|_| char::from_u32(rng.gen_range(97..123)).unwrap())
            .collect();
        format!("{}_{}.txt", prefix, chrono::Utc::now().timestamp())
    }
    
    /// 等待条件满足
    pub async fn wait_for_condition<F, Fut>(condition: F, timeout_ms: u64, interval_ms: u64) -> bool
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = bool>,
    {
        let start = std::time::Instant::now();
        let timeout = tokio::time::Duration::from_millis(timeout_ms);
        let interval = tokio::time::Duration::from_millis(interval_ms);
        
        while start.elapsed() < timeout {
            if condition().await {
                return true;
            }
            tokio::time::sleep(interval).await;
        }
        
        false
    }
}