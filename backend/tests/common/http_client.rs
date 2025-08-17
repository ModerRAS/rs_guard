//! HTTP 测试客户端
//! 
//! 这个模块提供了用于测试的 HTTP 客户端，包含：
//! - 请求构建器
//! - 响应断言
//! - 错误处理

use std::time::Duration;
use serde_json::Value;
use anyhow::Result;

/// HTTP 测试客户端
pub struct TestHttpClient {
    base_url: String,
    client: reqwest::Client,
    default_timeout: Duration,
}

impl TestHttpClient {
    /// 创建新的 HTTP 客户端
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: reqwest::Client::new(),
            default_timeout: Duration::from_secs(30),
        }
    }
    
    /// 设置默认超时时间
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.default_timeout = timeout;
        self
    }
    
    /// 设置默认头部
    pub fn with_default_headers(mut self, headers: &[(&str, &str)]) -> Self {
        let mut builder = reqwest::Client::builder();
        for (key, value) in headers {
            builder = builder.header(*key, *value);
        }
        self.client = builder.build().expect("Failed to build HTTP client");
        self
    }
    
    /// 发送 GET 请求
    pub async fn get(&self, endpoint: &str) -> Result<TestHttpResponse> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.get(&url)
            .timeout(self.default_timeout)
            .send()
            .await?;
        
        Ok(TestHttpResponse::from_reqwest(response).await)
    }
    
    /// 发送 POST 请求
    pub async fn post(&self, endpoint: &str, body: &Value) -> Result<TestHttpResponse> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.post(&url)
            .json(body)
            .timeout(self.default_timeout)
            .send()
            .await?;
        
        Ok(TestHttpResponse::from_reqwest(response).await)
    }
    
    /// 发送 POST 请求（表单数据）
    pub async fn post_form(&self, endpoint: &str, form: &[(&str, &str)]) -> Result<TestHttpResponse> {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut form_data = reqwest::multipart::Form::new();
        
        for (key, value) in form {
            form_data = form_data.text(*key, value.to_string());
        }
        
        let response = self.client.post(&url)
            .multipart(form_data)
            .timeout(self.default_timeout)
            .send()
            .await?;
        
        Ok(TestHttpResponse::from_reqwest(response).await)
    }
    
    /// 发送 PUT 请求
    pub async fn put(&self, endpoint: &str, body: &Value) -> Result<TestHttpResponse> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.put(&url)
            .json(body)
            .timeout(self.default_timeout)
            .send()
            .await?;
        
        Ok(TestHttpResponse::from_reqwest(response).await)
    }
    
    /// 发送 DELETE 请求
    pub async fn delete(&self, endpoint: &str) -> Result<TestHttpResponse> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.delete(&url)
            .timeout(self.default_timeout)
            .send()
            .await?;
        
        Ok(TestHttpResponse::from_reqwest(response).await)
    }
    
    /// 发送 PATCH 请求
    pub async fn patch(&self, endpoint: &str, body: &Value) -> Result<TestHttpResponse> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.patch(&url)
            .json(body)
            .timeout(self.default_timeout)
            .send()
            .await?;
        
        Ok(TestHttpResponse::from_reqwest(response).await)
    }
    
    /// 发送 HEAD 请求
    pub async fn head(&self, endpoint: &str) -> Result<TestHttpResponse> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.head(&url)
            .timeout(self.default_timeout)
            .send()
            .await?;
        
        Ok(TestHttpResponse::from_reqwest(response).await)
    }
    
    /// 发送 OPTIONS 请求
    pub async fn options(&self, endpoint: &str) -> Result<TestHttpResponse> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.options(&url)
            .timeout(self.default_timeout)
            .send()
            .await?;
        
        Ok(TestHttpResponse::from_reqwest(response).await)
    }
}

/// HTTP 响应包装器
pub struct TestHttpResponse {
    pub status: u16,
    pub headers: reqwest::header::HeaderMap,
    pub body: String,
    pub json: Option<Value>,
    pub duration: Duration,
}

impl TestHttpResponse {
    async fn from_reqwest(response: reqwest::Response) -> Self {
        let status = response.status().as_u16();
        let headers = response.headers().clone();
        
        let start = std::time::Instant::now();
        let body = response.text().await.unwrap_or_default();
        let duration = start.elapsed();
        
        let json = if body.trim().starts_with('{') || body.trim().starts_with('[') {
            serde_json::from_str(&body).ok()
        } else {
            None
        };

        Self { status, headers, body, json, duration }
    }
    
    /// 获取响应头
    pub fn header(&self, name: &str) -> Option<&reqwest::header::HeaderValue> {
        self.headers.get(name)
    }
    
    /// 获取响应头值
    pub fn header_value(&self, name: &str) -> Option<&str> {
        self.headers.get(name)?.to_str().ok()
    }
    
    /// 检查状态码
    pub fn assert_status(&self, expected: u16) -> &Self {
        assert_eq!(self.status, expected, "Expected status {}, got {}", expected, self.status);
        self
    }
    
    /// 检查状态码范围
    pub fn assert_status_range(&self, min: u16, max: u16) -> &Self {
        assert!(self.status >= min && self.status <= max, 
            "Expected status between {} and {}, got {}", min, max, self.status);
        self
    }
    
    /// 检查成功状态
    pub fn assert_success(&self) -> &Self {
        assert!(self.status >= 200 && self.status < 300, 
            "Expected success status (2xx), got {}", self.status);
        self
    }
    
    /// 检查客户端错误
    pub fn assert_client_error(&self) -> &Self {
        assert!(self.status >= 400 && self.status < 500, 
            "Expected client error status (4xx), got {}", self.status);
        self
    }
    
    /// 检查服务器错误
    pub fn assert_server_error(&self) -> &Self {
        assert!(self.status >= 500 && self.status < 600, 
            "Expected server error status (5xx), got {}", self.status);
        self
    }
    
    /// 检查响应体包含指定文本
    pub fn assert_body_contains(&self, text: &str) -> &Self {
        assert!(self.body.contains(text), "Response body should contain '{}'", text);
        self
    }
    
    /// 检查响应体不包含指定文本
    pub fn assert_body_not_contains(&self, text: &str) -> &Self {
        assert!(!self.body.contains(text), "Response body should not contain '{}'", text);
        self
    }
    
    /// 检查响应体为空
    pub fn assert_body_empty(&self) -> &Self {
        assert!(self.body.is_empty(), "Response body should be empty");
        self
    }
    
    /// 检查响应体不为空
    pub fn assert_body_not_empty(&self) -> &Self {
        assert!(!self.body.is_empty(), "Response body should not be empty");
        self
    }
    
    /// 检查响应体长度
    pub fn assert_body_length(&self, expected_length: usize) -> &Self {
        assert_eq!(self.body.len(), expected_length, 
            "Expected response body length {}, got {}", expected_length, self.body.len());
        self
    }
    
    /// 检查响应头
    pub fn assert_header(&self, name: &str, expected_value: &str) -> &Self {
        let actual_value = self.header_value(name)
            .unwrap_or_else(|| panic!("Header '{}' not found", name));
        assert_eq!(actual_value, expected_value, 
            "Header '{}' should be '{}', got '{}'", name, expected_value, actual_value);
        self
    }
    
    /// 检查响应头存在
    pub fn assert_header_exists(&self, name: &str) -> &Self {
        assert!(self.header(name).is_some(), "Header '{}' should exist", name);
        self
    }
    
    /// 检查 Content-Type 头
    pub fn assert_content_type(&self, expected_content_type: &str) -> &Self {
        self.assert_header("content-type", expected_content_type)
    }
    
    /// 检查是 JSON 响应
    pub fn assert_json(&self) -> &Value {
        self.json.as_ref().expect("Response is not valid JSON")
    }
    
    /// 检查 JSON 字段存在
    pub fn assert_json_field(&self, field: &str) -> &Value {
        let json = self.assert_json();
        json.get(field).expect(&format!("Field '{}' not found", field))
    }
    
    /// 检查 JSON 字段值
    pub fn assert_json_field_value(&self, field: &str, expected: &Value) -> &Self {
        let actual = self.assert_json_field(field);
        assert_eq!(actual, expected, "Field '{}' value mismatch", field);
        self
    }
    
    /// 检查 JSON 字段类型
    pub fn assert_json_field_type(&self, field: &str, expected_type: &str) -> &Self {
        let json = self.assert_json();
        let field_value = json.get(field)
            .unwrap_or_else(|| panic!("Field '{}' not found", field));
        
        let actual_type = match field_value {
            Value::Null => "null",
            Value::Bool(_) => "boolean",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
        };
        
        assert_eq!(actual_type, expected_type, 
            "Field '{}' should be type '{}', got '{}'", field, expected_type, actual_type);
        self
    }
    
    /// 检查 JSON 数组长度
    pub fn assert_json_array_length(&self, field: &str, expected_length: usize) -> &Self {
        let json = self.assert_json();
        let array = json.get(field)
            .unwrap_or_else(|| panic!("Field '{}' not found", field))
            .as_array()
            .unwrap_or_else(|| panic!("Field '{}' is not an array", field));
        
        assert_eq!(array.len(), expected_length, 
            "JSON array '{}' should have length {}, got {}", field, expected_length, array.len());
        self
    }
    
    /// 检查响应时间
    pub fn assert_response_time(&self, max_duration: Duration) -> &Self {
        assert!(self.duration <= max_duration, 
            "Response time should be <= {:?}, got {:?}", max_duration, self.duration);
        self
    }
    
    /// 获取 JSON 响应
    pub fn json(&self) -> Option<&Value> {
        self.json.as_ref()
    }
    
    /// 获取响应体
    pub fn body(&self) -> &str {
        &self.body
    }
    
    /// 获取状态码
    pub fn status(&self) -> u16 {
        self.status
    }
    
    /// 获取响应时间
    pub fn duration(&self) -> Duration {
        self.duration
    }
    
    /// 转换为 Result
    pub fn into_result(self) -> Result<Self, String> {
        if self.status >= 200 && self.status < 300 {
            Ok(self)
        } else {
            Err(format!("HTTP Error {}: {}", self.status, self.body))
        }
    }
}

impl std::fmt::Debug for TestHttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TestHttpResponse")
            .field("status", &self.status)
            .field("body_length", &self.body.len())
            .field("duration", &self.duration)
            .field("has_json", &self.json.is_some())
            .finish()
    }
}

/// 请求构建器
pub struct RequestBuilder {
    client: reqwest::Client,
    url: String,
    method: reqwest::Method,
    headers: reqwest::header::HeaderMap,
    timeout: Duration,
    json_body: Option<Value>,
}

impl RequestBuilder {
    pub fn new(method: reqwest::Method, url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            url,
            method,
            headers: reqwest::header::HeaderMap::new(),
            timeout: Duration::from_secs(30),
            json_body: None,
        }
    }
    
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key, value.parse().unwrap());
        self
    }
    
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    
    pub fn json(mut self, body: &Value) -> Self {
        self.headers.insert("content-type", "application/json".parse().unwrap());
        self.json_body = Some(body.clone());
        self
    }
    
    pub async fn send(self) -> Result<TestHttpResponse> {
        let mut request = self.client.request(self.method, &self.url)
            .timeout(self.timeout)
            .headers(self.headers);
        
        if let Some(body) = self.json_body {
            request = request.json(&body);
        }
        
        let response = request.send().await?;
        Ok(TestHttpResponse::from_reqwest(response).await)
    }
}

// 在 RequestBuilder 中添加字段
impl RequestBuilder {
    pub fn json_body(mut self, json_body: Option<Value>) -> Self {
        self.json_body = json_body;
        self
    }
}

impl Default for RequestBuilder {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            url: String::new(),
            method: reqwest::Method::GET,
            headers: reqwest::header::HeaderMap::new(),
            timeout: Duration::from_secs(30),
            json_body: None,
        }
    }
}