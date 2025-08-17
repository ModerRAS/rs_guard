//! BDD 测试工具函数
//! 
//! 这个模块提供了 BDD 测试中常用的工具函数和辅助方法。

use std::path::{Path, PathBuf};
use std::fs;
use serde_json::Value;
use tokio::time::{sleep, Duration};
use anyhow::Result;

/// 测试数据生成器
pub struct TestDataGenerator {
    base_dir: PathBuf,
}

impl TestDataGenerator {
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }

    /// 创建测试文件
    pub async fn create_test_file(&self, filename: &str, content: &str) -> Result<PathBuf> {
        let file_path = self.base_dir.join(filename);
        
        // 确保目录存在
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // 写入文件
        fs::write(&file_path, content)?;
        
        Ok(file_path)
    }

    /// 创建二进制测试文件
    pub async fn create_binary_test_file(&self, filename: &str, size: usize) -> Result<PathBuf> {
        let file_path = self.base_dir.join(filename);
        
        // 确保目录存在
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // 生成随机二进制数据
        let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        fs::write(&file_path, data)?;
        
        Ok(file_path)
    }

    /// 创建测试目录结构
    pub async fn create_directory_structure(&self, structure: &[&str]) -> Result<()> {
        for dir in structure {
            let dir_path = self.base_dir.join(dir);
            fs::create_dir_all(&dir_path)?;
        }
        Ok(())
    }

    /// 获取文件大小
    pub fn get_file_size(&self, filename: &str) -> Result<u64> {
        let file_path = self.base_dir.join(filename);
        Ok(fs::metadata(&file_path)?.len())
    }

    /// 检查文件是否存在
    pub fn file_exists(&self, filename: &str) -> bool {
        let file_path = self.base_dir.join(filename);
        file_path.exists()
    }

    /// 删除测试文件
    pub async fn cleanup_file(&self, filename: &str) -> Result<()> {
        let file_path = self.base_dir.join(filename);
        if file_path.exists() {
            fs::remove_file(&file_path)?;
        }
        Ok(())
    }

    /// 清理所有测试文件
    pub async fn cleanup_all(&self) -> Result<()> {
        if self.base_dir.exists() {
            fs::remove_dir_all(&self.base_dir)?;
        }
        Ok(())
    }
}

/// HTTP 测试客户端
pub struct TestHttpClient {
    base_url: String,
    client: reqwest::Client,
}

impl TestHttpClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: reqwest::Client::new(),
        }
    }

    /// 发送 GET 请求
    pub async fn get(&self, endpoint: &str) -> Result<TestHttpResponse> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.get(&url).send().await?;
        
        Ok(TestHttpResponse::from_reqwest(response).await)
    }

    /// 发送 POST 请求
    pub async fn post(&self, endpoint: &str, body: &Value) -> Result<TestHttpResponse> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.post(&url).json(body).send().await?;
        
        Ok(TestHttpResponse::from_reqwest(response).await)
    }

    /// 发送 PUT 请求
    pub async fn put(&self, endpoint: &str, body: &Value) -> Result<TestHttpResponse> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.put(&url).json(body).send().await?;
        
        Ok(TestHttpResponse::from_reqwest(response).await)
    }

    /// 发送 DELETE 请求
    pub async fn delete(&self, endpoint: &str) -> Result<TestHttpResponse> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.delete(&url).send().await?;
        
        Ok(TestHttpResponse::from_reqwest(response).await)
    }
}

/// HTTP 响应包装器
pub struct TestHttpResponse {
    pub status: u16,
    pub body: String,
    pub json: Option<Value>,
}

impl TestHttpResponse {
    async fn from_reqwest(response: reqwest::Response) -> Self {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        
        let json = if body.trim().starts_with('{') || body.trim().starts_with('[') {
            serde_json::from_str(&body).ok()
        } else {
            None
        };

        Self { status, body, json }
    }

    /// 检查状态码
    pub fn assert_status(&self, expected: u16) -> &Self {
        assert_eq!(self.status, expected, "Expected status {}, got {}", expected, self.status);
        self
    }

    /// 检查响应体包含指定文本
    pub fn assert_body_contains(&self, text: &str) -> &Self {
        assert!(self.body.contains(text), "Response body should contain '{}'", text);
        self
    }

    /// 检查 JSON 响应
    pub fn assert_json(&self) -> &Value {
        self.json.as_ref().expect("Response is not valid JSON")
    }

    /// 检查 JSON 字段
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
}

/// 断言工具
pub struct AssertUtils;

impl AssertUtils {
    /// 断言文件存在
    pub fn assert_file_exists<P: AsRef<Path>>(path: P) -> &Path {
        let path = path.as_ref();
        assert!(path.exists(), "File '{}' should exist", path.display());
        path
    }

    /// 断言文件不存在
    pub fn assert_file_not_exists<P: AsRef<Path>>(path: P) -> &Path {
        let path = path.as_ref();
        assert!(!path.exists(), "File '{}' should not exist", path.display());
        path
    }

    /// 断言目录存在
    pub fn assert_dir_exists<P: AsRef<Path>>(path: P) -> &Path {
        let path = path.as_ref();
        assert!(path.exists() && path.is_dir(), "Directory '{}' should exist", path.display());
        path
    }

    /// 断言文件内容
    pub async fn assert_file_content<P: AsRef<Path>>(path: P, expected_content: &str) -> Result<()> {
        let path = path.as_ref();
        let actual_content = fs::read_to_string(path)?;
        assert!(actual_content.contains(expected_content), 
            "File '{}' should contain '{}', got: '{}'", 
            path.display(), expected_content, actual_content);
        Ok(())
    }

    /// 断言文件大小
    pub fn assert_file_size<P: AsRef<Path>>(path: P, expected_size: u64) -> Result<()> {
        let path = path.as_ref();
        let actual_size = fs::metadata(path)?.len();
        assert_eq!(actual_size, expected_size, 
            "File '{}' size should be {}, got {}", 
            path.display(), expected_size, actual_size);
        Ok(())
    }
}

/// 等待工具
pub struct WaitUtils;

impl WaitUtils {
    /// 等待条件满足
    pub async fn wait_for<F, Fut>(condition: F, timeout_ms: u64, interval_ms: u64) -> Result<()>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = bool>,
    {
        let start_time = std::time::Instant::now();
        let timeout = Duration::from_millis(timeout_ms);
        let interval = Duration::from_millis(interval_ms);

        loop {
            if condition().await {
                return Ok(());
            }

            if start_time.elapsed() >= timeout {
                return Err(anyhow::anyhow!("Condition not met within {}ms timeout", timeout_ms));
            }

            sleep(interval).await;
        }
    }

    /// 等待文件出现
    pub async fn wait_for_file<P: AsRef<Path>>(path: P, timeout_ms: u64) -> Result<()> {
        let path = path.as_ref().to_path_buf();
        Self::wait_for(
            || async { path.exists() },
            timeout_ms,
            100,
        ).await
    }

    /// 等待文件消失
    pub async fn wait_for_file_not_exists<P: AsRef<Path>>(path: P, timeout_ms: u64) -> Result<()> {
        let path = path.as_ref().to_path_buf();
        Self::wait_for(
            || async { !path.exists() },
            timeout_ms,
            100,
        ).await
    }

    /// 等待 HTTP 服务可用
    pub async fn wait_for_service(url: &str, timeout_ms: u64) -> Result<()> {
        let client = reqwest::Client::new();
        let url = url.to_string();
        
        Self::wait_for(
            || async {
                client.get(&url).send().await
                    .map(|resp| resp.status().is_success())
                    .unwrap_or(false)
            },
            timeout_ms,
            500,
        ).await
    }
}

/// 测试环境管理器
pub struct TestEnvManager {
    temp_dir: Option<PathBuf>,
    test_data_dir: Option<PathBuf>,
}

impl TestEnvManager {
    pub fn new() -> Self {
        Self {
            temp_dir: None,
            test_data_dir: None,
        }
    }

    /// 创建临时测试环境
    pub fn setup(&mut self) -> Result<PathBuf> {
        let temp_dir = tempfile::tempdir()?;
        let temp_path = temp_dir.into_path();
        self.temp_dir = Some(temp_path.clone());
        
        Ok(temp_path)
    }

    /// 创建测试数据目录
    pub fn create_test_data_dir(&mut self, base_dir: &Path) -> Result<PathBuf> {
        let test_data_dir = base_dir.join("test-data");
        fs::create_dir_all(&test_data_dir)?;
        self.test_data_dir = Some(test_data_dir.clone());
        
        Ok(test_data_dir)
    }

    /// 获取临时目录
    pub fn temp_dir(&self) -> Option<&PathBuf> {
        self.temp_dir.as_ref()
    }

    /// 获取测试数据目录
    pub fn test_data_dir(&self) -> Option<&PathBuf> {
        self.test_data_dir.as_ref()
    }

    /// 清理测试环境
    pub async fn cleanup(&mut self) -> Result<()> {
        if let Some(temp_dir) = &self.temp_dir {
            if temp_dir.exists() {
                fs::remove_dir_all(temp_dir)?;
            }
        }
        Ok(())
    }
}

impl Default for TestEnvManager {
    fn default() -> Self {
        Self::new()
    }
}