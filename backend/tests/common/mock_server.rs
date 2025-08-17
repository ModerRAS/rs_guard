//! 模拟服务器
//! 
//! 这个模块提供了用于测试的模拟服务器，可以模拟各种 HTTP 响应。

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use axum::{extract, response::Json, routing, Router};
use serde_json::Value;
use tokio::net::TcpListener;
use anyhow::Result;

/// 模拟响应配置
#[derive(Debug, Clone)]
pub struct MockResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub delay_ms: u64,
}

impl Default for MockResponse {
    fn default() -> Self {
        Self {
            status: 200,
            headers: HashMap::new(),
            body: String::new(),
            delay_ms: 0,
        }
    }
}

impl MockResponse {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn status(mut self, status: u16) -> Self {
        self.status = status;
        self
    }
    
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }
    
    pub fn body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }
    
    pub fn json_body(mut self, body: &Value) -> Self {
        self.body = serde_json::to_string(body).unwrap();
        self.headers.insert("content-type".to_string(), "application/json".to_string());
        self
    }
    
    pub fn delay(mut self, delay_ms: u64) -> Self {
        self.delay_ms = delay_ms;
        self
    }
    
    pub fn success() -> Self {
        Self::new().status(200)
    }
    
    pub fn not_found() -> Self {
        Self::new().status(404).body("Not Found")
    }
    
    pub fn server_error() -> Self {
        Self::new().status(500).body("Internal Server Error")
    }
    
    pub fn bad_request() -> Self {
        Self::new().status(400).body("Bad Request")
    }
    
    pub fn unauthorized() -> Self {
        Self::new().status(401).body("Unauthorized")
    }
    
    pub fn forbidden() -> Self {
        Self::new().status(403).body("Forbidden")
    }
}

/// 模拟端点
#[derive(Debug, Clone)]
pub struct MockEndpoint {
    pub path: String,
    pub method: String,
    pub response: MockResponse,
    pub request_count: Arc<Mutex<usize>>,
}

impl MockEndpoint {
    pub fn new(path: &str, method: &str) -> Self {
        Self {
            path: path.to_string(),
            method: method.to_string(),
            response: MockResponse::default(),
            request_count: Arc::new(Mutex::new(0)),
        }
    }
    
    pub fn response(mut self, response: MockResponse) -> Self {
        self.response = response;
        self
    }
    
    pub fn increment_request_count(&self) {
        let mut count = self.request_count.lock().unwrap();
        *count += 1;
    }
    
    pub fn request_count(&self) -> usize {
        *self.request_count.lock().unwrap()
    }
    
    pub fn reset_request_count(&self) {
        let mut count = self.request_count.lock().unwrap();
        *count = 0;
    }
}

/// 模拟服务器
pub struct MockServer {
    endpoints: Vec<MockEndpoint>,
    address: Option<String>,
    handle: Option<tokio::task::JoinHandle<()>>,
}

impl MockServer {
    pub fn new() -> Self {
        Self {
            endpoints: Vec::new(),
            address: None,
            handle: None,
        }
    }
    
    /// 添加模拟端点
    pub fn add_endpoint(mut self, endpoint: MockEndpoint) -> Self {
        self.endpoints.push(endpoint);
        self
    }
    
    /// 添加 GET 端点
    pub fn get(self, path: &str, response: MockResponse) -> Self {
        self.add_endpoint(MockEndpoint::new(path, "GET").response(response))
    }
    
    /// 添加 POST 端点
    pub fn post(self, path: &str, response: MockResponse) -> Self {
        self.add_endpoint(MockEndpoint::new(path, "POST").response(response))
    }
    
    /// 添加 PUT 端点
    pub fn put(self, path: &str, response: MockResponse) -> Self {
        self.add_endpoint(MockEndpoint::new(path, "PUT").response(response))
    }
    
    /// 添加 DELETE 端点
    pub fn delete(self, path: &str, response: MockResponse) -> Self {
        self.add_endpoint(MockEndpoint::new(path, "DELETE").response(response))
    }
    
    /// 启动模拟服务器
    pub async fn start(mut self) -> Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        let server_address = format!("http://{}", addr);
        
        // 创建路由
        let app = self.create_router();
        
        // 启动服务器
        let handle = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        
        self.address = Some(server_address);
        self.handle = Some(handle);
        
        // 等待服务器启动
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        Ok(self)
    }
    
    /// 创建路由
    fn create_router(&self) -> Router {
        let mut router = Router::new();
        
        for endpoint in &self.endpoints {
            let endpoint = endpoint.clone();
            
            router = match endpoint.method.as_str() {
                "GET" => router.route(&endpoint.path, routing::get({
                    let endpoint = endpoint.clone();
                    move || async move { self.handle_request(endpoint).await }
                })),
                "POST" => router.route(&endpoint.path, routing::post({
                    let endpoint = endpoint.clone();
                    move || async move { self.handle_request(endpoint).await }
                })),
                "PUT" => router.route(&endpoint.path, routing::put({
                    let endpoint = endpoint.clone();
                    move || async move { self.handle_request(endpoint).await }
                })),
                "DELETE" => router.route(&endpoint.path, routing::delete({
                    let endpoint = endpoint.clone();
                    move || async move { self.handle_request(endpoint).await }
                })),
                _ => router,
            };
        }
        
        router
    }
    
    /// 处理请求
    async fn handle_request(&self, endpoint: MockEndpoint) -> axum::response::Response {
        // 增加请求计数
        endpoint.increment_request_count();
        
        // 如果需要延迟
        if endpoint.response.delay_ms > 0 {
            tokio::time::sleep(tokio::time::Duration::from_millis(endpoint.response.delay_ms)).await;
        }
        
        // 构建响应
        let mut response = axum::response::Response::builder()
            .status(endpoint.response.status);
        
        // 添加响应头
        for (key, value) in &endpoint.response.headers {
            response = response.header(key, value);
        }
        
        // 设置响应体
        let body = endpoint.response.body.clone();
        response.body(body).unwrap()
    }
    
    /// 获取服务器地址
    pub fn address(&self) -> &str {
        self.address.as_deref().unwrap_or("")
    }
    
    /// 获取端点的请求计数
    pub fn request_count(&self, path: &str, method: &str) -> Option<usize> {
        self.endpoints
            .iter()
            .find(|e| e.path == path && e.method == method)
            .map(|e| e.request_count())
    }
    
    /// 重置所有端点的请求计数
    pub fn reset_all_request_counts(&self) {
        for endpoint in &self.endpoints {
            endpoint.reset_request_count();
        }
    }
    
    /// 停止服务器
    pub async fn stop(self) {
        if let Some(handle) = self.handle {
            handle.abort();
        }
    }
}

impl Default for MockServer {
    fn default() -> Self {
        Self::new()
    }
}

/// 模拟服务器构建器
pub struct MockServerBuilder {
    server: MockServer,
}

impl MockServerBuilder {
    pub fn new() -> Self {
        Self {
            server: MockServer::new(),
        }
    }
    
    pub fn get(mut self, path: &str, response: MockResponse) -> Self {
        self.server = self.server.get(path, response);
        self
    }
    
    pub fn post(mut self, path: &str, response: MockResponse) -> Self {
        self.server = self.server.post(path, response);
        self
    }
    
    pub fn put(mut self, path: &str, response: MockResponse) -> Self {
        self.server = self.server.put(path, response);
        self
    }
    
    pub fn delete(mut self, path: &str, response: MockResponse) -> Self {
        self.server = self.server.delete(path, response);
        self
    }
    
    pub async fn start(self) -> Result<MockServer> {
        self.server.start().await
    }
}

impl Default for MockServerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 预定义的模拟响应
pub struct MockResponses;

impl MockResponses {
    /// 成功响应
    pub fn success() -> MockResponse {
        MockResponse::success()
            .header("content-type", "application/json")
            .json_body(&serde_json::json!({
                "status": "success",
                "message": "Operation completed successfully"
            }))
    }
    
    /// 错误响应
    pub fn error(message: &str) -> MockResponse {
        MockResponse::bad_request()
            .header("content-type", "application/json")
            .json_body(&serde_json::json!({
                "status": "error",
                "message": message
            }))
    }
    
    /// 文件列表响应
    pub fn file_list(files: Vec<&str>) -> MockResponse {
        MockResponse::success()
            .header("content-type", "application/json")
            .json_body(&serde_json::json!({
                "files": files,
                "total": files.len()
            }))
    }
    
    /// 状态响应
    pub fn status(total_files: usize, protected_files: usize, corrupted_files: usize) -> MockResponse {
        MockResponse::success()
            .header("content-type", "application/json")
            .json_body(&serde_json::json!({
                "total_files": total_files,
                "protected_files": protected_files,
                "corrupted_files": corrupted_files,
                "last_check": chrono::Utc::now().to_rfc3339()
            }))
    }
    
    /// 检查响应
    pub fn check_result(checked_files: usize, corrupted_files: usize) -> MockResponse {
        MockResponse::success()
            .header("content-type", "application/json")
            .json_body(&serde_json::json!({
                "status": if corrupted_files == 0 { "healthy" } else { "corrupted" },
                "checked_files": checked_files,
                "corrupted_files": corrupted_files,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
    }
    
    /// 配置响应
    pub fn config(watched_dirs: Vec<&str>, data_shards: usize, parity_shards: usize) -> MockResponse {
        MockResponse::success()
            .header("content-type", "application/json")
            .json_body(&serde_json::json!({
                "watched_directories": watched_dirs,
                "data_shards": data_shards,
                "parity_shards": parity_shards
            }))
    }
    
    /// 延迟响应
    pub fn delayed(response: MockResponse, delay_ms: u64) -> MockResponse {
        response.delay(delay_ms)
    }
    
    /// 空响应
    pub fn empty() -> MockResponse {
        MockResponse::success().body("")
    }
    
    /// 大响应
    pub fn large_response(size: usize) -> MockResponse {
        let content = "x".repeat(size);
        MockResponse::success()
            .header("content-type", "text/plain")
            .body(&content)
    }
}

/// 便捷函数：快速创建模拟服务器
pub async fn create_mock_server() -> Result<MockServer> {
    MockServer::new()
        .get("/api/status", MockResponses::status(10, 8, 2))
        .get("/api/files", MockResponses::file_list(vec!["file1.txt", "file2.txt"]))
        .get("/api/check", MockResponses::check_result(10, 2))
        .get("/api/config", MockResponses::config(vec!["/test/data"], 4, 2))
        .start()
        .await
}

/// 便捷函数：创建错误模拟服务器
pub async fn create_error_mock_server() -> Result<MockServer> {
    MockServer::new()
        .get("/api/status", MockResponses::error("Server error"))
        .post("/api/files", MockResponses::error("Invalid request"))
        .start()
        .await
}

/// 便捷函数：创建延迟模拟服务器
pub async fn create_delayed_mock_server() -> Result<MockServer> {
    MockServer::new()
        .get("/api/status", MockResponses::delayed(MockResponses::status(10, 8, 2), 1000))
        .get("/api/files", MockResponses::delayed(MockResponses::file_list(vec!["file1.txt"]), 500))
        .start()
        .await
}