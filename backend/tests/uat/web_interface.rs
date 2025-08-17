//! Web 界面用户验收测试
//! 
//! 这个模块包含 Web 界面功能的用户验收测试，验证：
//! - API 响应正确性
//! - 状态页面显示
//! - 文件列表功能
//! - 配置管理接口

use super::*;
use serde_json::json;

/// Web 界面测试套件
pub struct WebInterfaceTests;

impl WebInterfaceTests {
    /// 测试状态 API
    pub async fn test_status_api() {
        let config = UatConfig::default();
        let context = UatContext::new(config).await;
        
        println!("测试场景：状态 API");
        
        let client = reqwest::Client::new();
        
        // 测试状态端点
        let response = client
            .get(&format!("{}/api/status", context.server_url()))
            .send()
            .await
            .expect("Failed to get status");
        
        UatAssertions::assert_status(&response, 200);
        let status = UatAssertions::assert_json(response).await;
        
        // 验证状态字段
        UatAssertions::assert_json_field(&status, "data_shards");
        UatAssertions::assert_json_field(&status, "parity_shards");
        UatAssertions::assert_json_field(&status, "watched_dirs");
        UatAssertions::assert_json_field(&status, "total_files");
        UatAssertions::assert_json_field(&status, "protected_files");
        UatAssertions::assert_json_field(&status, "corrupted_files");
        UatAssertions::assert_json_field(&status, "last_check");
        
        // 验证字段值
        UatAssertions::assert_json_field_value(&status, "data_shards", &json!(4));
        UatAssertions::assert_json_field_value(&status, "parity_shards", &json!(2));
        
        // 清理
        context.cleanup().await;
    }
    
    /// 测试文件列表 API
    pub async fn test_files_api() {
        let config = UatConfig::default();
        let context = UatContext::new(config).await;
        
        println!("测试场景：文件列表 API");
        
        // 创建测试文件
        context.create_test_file("api_test_1.txt", "API 测试文件 1").await;
        context.create_test_file("api_test_2.txt", "API 测试文件 2").await;
        
        // 等待文件处理
        context.wait_for_file_processing(3000).await;
        
        let client = reqwest::Client::new();
        
        // 测试文件列表端点
        let response = client
            .get(&format!("{}/api/files", context.server_url()))
            .send()
            .await
            .expect("Failed to get files");
        
        UatAssertions::assert_status(&response, 200);
        let files_response = UatAssertions::assert_json(response).await;
        
        // 验证文件列表字段
        UatAssertions::assert_json_field(&files_response, "files");
        UatAssertions::assert_json_field(&files_response, "total");
        
        // 验证文件列表是数组
        let files = &files_response["files"];
        assert!(files.is_array(), "files 应该是一个数组");
        
        // 清理
        context.cleanup().await;
    }
    
    /// 测试检查 API
    pub async fn test_check_api() {
        let config = UatConfig::default();
        let context = UatContext::new(config).await;
        
        println!("测试场景：检查 API");
        
        let client = reqwest::Client::new();
        
        // 测试检查端点
        let response = client
            .get(&format!("{}/api/check", context.server_url()))
            .send()
            .await
            .expect("Failed to check");
        
        UatAssertions::assert_status(&response, 200);
        let check_response = UatAssertions::assert_json(response).await;
        
        // 验证检查响应字段
        UatAssertions::assert_json_field(&check_response, "status");
        UatAssertions::assert_json_field(&check_response, "checked_files");
        UatAssertions::assert_json_field(&check_response, "corrupted_files");
        UatAssertions::assert_json_field(&check_response, "timestamp");
        
        // 清理
        context.cleanup().await;
    }
    
    /// 测试配置 API
    pub async fn test_config_api() {
        let config = UatConfig::default();
        let context = UatContext::new(config).await;
        
        println!("测试场景：配置 API");
        
        let client = reqwest::Client::new();
        
        // 测试配置端点
        let response = client
            .get(&format!("{}/api/config", context.server_url()))
            .send()
            .await
            .expect("Failed to get config");
        
        UatAssertions::assert_status(&response, 200);
        let config_response = UatAssertions::assert_json(response).await;
        
        // 验证配置字段
        UatAssertions::assert_json_field(&config_response, "watched_directories");
        UatAssertions::assert_json_field(&config_response, "data_shards");
        UatAssertions::assert_json_field(&config_response, "parity_shards");
        
        // 清理
        context.cleanup().await;
    }
    
    /// 测试 API 错误处理
    pub async fn test_api_error_handling() {
        let config = UatConfig::default();
        let context = UatContext::new(config).await;
        
        println!("测试场景：API 错误处理");
        
        let client = reqwest::Client::new();
        
        // 测试不存在的端点
        let response = client
            .get(&format!("{}/api/nonexistent", context.server_url()))
            .send()
            .await
            .expect("Failed to send request");
        
        UatAssertions::assert_status(&response, 404);
        
        // 测试无效方法
        let response = client
            .post(&format!("{}/api/status", context.server_url()))
            .send()
            .await
            .expect("Failed to send request");
        
        // 应该返回 405 Method Not Allowed 或适当的错误状态
        assert!(!response.status().is_success(), "POST to status endpoint should fail");
        
        // 清理
        context.cleanup().await;
    }
    
    /// 测试 API 响应时间
    pub async fn test_api_response_time() {
        let config = UatConfig::default();
        let context = UatContext::new(config).await;
        
        println!("测试场景：API 响应时间");
        
        let client = reqwest::Client::new();
        
        // 测试状态 API 响应时间
        let start = std::time::Instant::now();
        let response = client
            .get(&format!("{}/api/status", context.server_url()))
            .send()
            .await
            .expect("Failed to get status");
        let duration = start.elapsed();
        
        UatAssertions::assert_status(&response, 200);
        assert!(duration.as_millis() < 1000, "状态 API 响应时间应该小于 1 秒");
        
        // 测试文件列表 API 响应时间
        let start = std::time::Instant::now();
        let response = client
            .get(&format!("{}/api/files", context.server_url()))
            .send()
            .await
            .expect("Failed to get files");
        let duration = start.elapsed();
        
        UatAssertions::assert_status(&response, 200);
        assert!(duration.as_millis() < 1000, "文件列表 API 响应时间应该小于 1 秒");
        
        // 清理
        context.cleanup().await;
    }
    
    /// 测试并发 API 请求
    pub async fn test_concurrent_api_requests() {
        let config = UatConfig::default();
        let context = UatContext::new(config).await;
        
        println!("测试场景：并发 API 请求");
        
        let client = reqwest::Client::new();
        let server_url = context.server_url();
        
        // 创建多个并发请求
        let mut handles = vec![];
        for i in 0..10 {
            let url = format!("{}/api/status", server_url);
            let client = client.clone();
            let handle = tokio::spawn(async move {
                let response = client.get(&url).send().await.expect("Failed to send request");
                assert!(response.status().is_success());
                i
            });
            handles.push(handle);
        }
        
        // 等待所有请求完成
        for handle in handles {
            let result = handle.await.expect("Failed to wait for request");
            println!("并发请求 {} 完成", result);
        }
        
        // 清理
        context.cleanup().await;
    }
    
    /// 测试静态文件服务
    pub async fn test_static_files_serving() {
        let config = UatConfig::default();
        let context = UatContext::new(config).await;
        
        println!("测试场景：静态文件服务");
        
        let client = reqwest::Client::new();
        
        // 测试根路径（应该返回前端页面）
        let response = client
            .get(&context.server_url())
            .send()
            .await
            .expect("Failed to get root");
        
        UatAssertions::assert_status(&response, 200);
        
        // 验证响应是 HTML
        let content_type = response.headers().get("content-type");
        assert!(content_type.is_some(), "应该有 content-type 头");
        let content_type = content_type.unwrap().to_str().unwrap();
        assert!(content_type.contains("text/html"), "响应应该是 HTML");
        
        // 清理
        context.cleanup().await;
    }
    
    /// 运行所有 Web 界面测试
    pub async fn run_all_tests() {
        println!("🌐 开始运行 Web 界面用户验收测试...");
        
        Self::test_status_api().await;
        println!("✅ 状态 API 测试通过");
        
        Self::test_files_api().await;
        println!("✅ 文件列表 API 测试通过");
        
        Self::test_check_api().await;
        println!("✅ 检查 API 测试通过");
        
        Self::test_config_api().await;
        println!("✅ 配置 API 测试通过");
        
        Self::test_api_error_handling().await;
        println!("✅ API 错误处理测试通过");
        
        Self::test_api_response_time().await;
        println!("✅ API 响应时间测试通过");
        
        Self::test_concurrent_api_requests().await;
        println!("✅ 并发 API 请求测试通过");
        
        Self::test_static_files_serving().await;
        println!("✅ 静态文件服务测试通过");
        
        println!("🎉 所有 Web 界面用户验收测试通过！");
    }
}

#[tokio::test]
async fn test_web_interface_uat() {
    WebInterfaceTests::run_all_tests().await;
}