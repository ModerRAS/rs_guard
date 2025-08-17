//! 测试环境管理
//! 
//! 这个模块提供了测试环境的创建、配置和清理功能。

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use backend::{config, metadata, app_router};
use shared::AppStatus;
use tempfile::tempdir;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use anyhow::Result;

/// 测试环境管理器
pub struct TestEnvironment {
    /// 运行时
    runtime: Runtime,
    /// 配置
    config: TestConfig,
    /// 服务器地址
    server_address: Option<SocketAddr>,
    /// 临时目录
    temp_dir: Option<PathBuf>,
    /// 测试数据目录
    test_data_dir: Option<PathBuf>,
    /// 应用状态
    app_state: Option<Arc<Mutex<AppStatus>>>,
    /// 数据库实例
    db: Option<Arc<metadata::Database>>,
}

impl TestEnvironment {
    /// 创建新的测试环境
    pub fn new(config: TestConfig) -> Result<Self> {
        let runtime = Runtime::new()?;
        
        Ok(Self {
            runtime,
            config,
            server_address: None,
            temp_dir: None,
            test_data_dir: None,
            app_state: None,
            db: None,
        })
    }
    
    /// 设置测试环境
    pub async fn setup(&mut self) -> Result<()> {
        // 创建临时目录
        let temp_dir = tempdir()?;
        let temp_path = temp_dir.into_path();
        self.temp_dir = Some(temp_path.clone());
        
        // 创建测试数据目录结构
        let test_data_dir = temp_path.join("test-data");
        let source_dir = test_data_dir.join("source");
        let backup_dir = test_data_dir.join("backup");
        
        tokio::fs::create_dir_all(&source_dir).await?;
        tokio::fs::create_dir_all(&backup_dir).await?;
        
        self.test_data_dir = Some(test_data_dir.clone());
        
        // 创建应用配置
        let app_config = config::AppConfig {
            watched_directories: vec![source_dir.to_string_lossy().to_string()],
            data_shards: self.config.data_shards,
            parity_shards: self.config.parity_shards,
        };
        
        // 创建临时数据库
        let db_path = temp_path.join("test_db");
        let db = Arc::new(metadata::open_db(db_path.to_str().unwrap())?);
        self.db = Some(db.clone());
        
        // 创建应用状态
        let app_state = Arc::new(Mutex::new(AppStatus {
            watched_dirs: vec![source_dir.to_string_lossy().to_string()],
            data_shards: app_config.data_shards,
            parity_shards: app_config.parity_shards,
            ..Default::default()
        }));
        self.app_state = Some(app_state.clone());
        
        // 启动服务器
        let listener = TcpListener::bind(&format!("127.0.0.1:{}", self.config.server_port))
            .await?;
        let server_address = listener.local_addr().unwrap();
        self.server_address = Some(server_address);
        
        // 构建应用路由
        let app = app_router(app_state, db);
        
        // 在后台启动服务器
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        
        // 等待服务器启动
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        Ok(())
    }
    
    /// 获取服务器 URL
    pub fn server_url(&self) -> String {
        match self.server_address {
            Some(addr) => format!("http://{}", addr),
            None => panic!("Test environment not set up"),
        }
    }
    
    /// 获取监控目录路径
    pub fn watched_dir(&self) -> PathBuf {
        match &self.test_data_dir {
            Some(test_data_dir) => test_data_dir.join("source"),
            None => panic!("Test environment not set up"),
        }
    }
    
    /// 获取备份目录路径
    pub fn backup_dir(&self) -> PathBuf {
        match &self.test_data_dir {
            Some(test_data_dir) => test_data_dir.join("backup"),
            None => panic!("Test environment not set up"),
        }
    }
    
    /// 获取运行时引用
    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }
    
    /// 获取应用状态
    pub fn app_state(&self) -> &Arc<Mutex<AppStatus>> {
        self.app_state.as_ref().expect("Test environment not set up")
    }
    
    /// 获取数据库引用
    pub fn db(&self) -> &Arc<metadata::Database> {
        self.db.as_ref().expect("Test environment not set up")
    }
    
    /// 创建测试文件
    pub async fn create_test_file(&self, filename: &str, content: &str) -> Result<PathBuf> {
        let file_path = self.watched_dir().join(filename);
        
        // 确保目录存在
        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        // 写入文件
        tokio::fs::write(&file_path, content).await?;
        
        Ok(file_path)
    }
    
    /// 创建二进制测试文件
    pub async fn create_binary_test_file(&self, filename: &str, size: usize) -> Result<PathBuf> {
        let file_path = self.watched_dir().join(filename);
        
        // 确保目录存在
        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        // 生成随机二进制数据
        let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        tokio::fs::write(&file_path, data).await?;
        
        Ok(file_path)
    }
    
    /// 检查文件是否存在
    pub async fn file_exists(&self, filename: &str) -> bool {
        let file_path = self.watched_dir().join(filename);
        tokio::fs::metadata(&file_path).await.is_ok()
    }
    
    /// 读取文件内容
    pub async fn read_file(&self, filename: &str) -> Result<String> {
        let file_path = self.watched_dir().join(filename);
        Ok(tokio::fs::read_to_string(&file_path).await?)
    }
    
    /// 删除文件
    pub async fn delete_file(&self, filename: &str) -> Result<()> {
        let file_path = self.watched_dir().join(filename);
        tokio::fs::remove_file(&file_path).await?;
        Ok(())
    }
    
    /// 获取文件大小
    pub async fn file_size(&self, filename: &str) -> Result<u64> {
        let file_path = self.watched_dir().join(filename);
        let metadata = tokio::fs::metadata(&file_path).await?;
        Ok(metadata.len())
    }
    
    /// 等待文件处理完成
    pub async fn wait_for_file_processing(&self, timeout_ms: u64) -> bool {
        let start = std::time::Instant::now();
        let timeout = tokio::time::Duration::from_millis(timeout_ms);
        
        while start.elapsed() < timeout {
            // 检查应用状态是否更新
            let state = self.app_state().lock().unwrap();
            if state.total_files > 0 {
                return true;
            }
            drop(state);
            
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        
        false
    }
    
    /// 等待条件满足
    pub async fn wait_for_condition<F, Fut>(&self, condition: F, timeout_ms: u64, interval_ms: u64) -> bool
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
    
    /// 执行 HTTP 请求
    pub async fn http_get(&self, endpoint: &str) -> Result<reqwest::Response> {
        let client = reqwest::Client::new();
        let url = format!("{}{}", self.server_url(), endpoint);
        let response = client.get(&url)
            .timeout(tokio::time::Duration::from_millis(self.config.request_timeout_ms))
            .send()
            .await?;
        Ok(response)
    }
    
    /// 执行 HTTP POST 请求
    pub async fn http_post(&self, endpoint: &str, body: &serde_json::Value) -> Result<reqwest::Response> {
        let client = reqwest::Client::new();
        let url = format!("{}{}", self.server_url(), endpoint);
        let response = client.post(&url)
            .json(body)
            .timeout(tokio::time::Duration::from_millis(self.config.request_timeout_ms))
            .send()
            .await?;
        Ok(response)
    }
    
    /// 获取应用状态快照
    pub fn get_app_state_snapshot(&self) -> AppStatus {
        self.app_state().lock().unwrap().clone()
    }
    
    /// 重置应用状态
    pub fn reset_app_state(&self) {
        let mut state = self.app_state().lock().unwrap();
        *state = AppStatus {
            watched_dirs: state.watched_dirs.clone(),
            data_shards: state.data_shards,
            parity_shards: state.parity_shards,
            ..Default::default()
        };
    }
    
    /// 清理测试环境
    pub async fn cleanup(mut self) -> Result<()> {
        if let Some(temp_dir) = self.temp_dir {
            if temp_dir.exists() {
                tokio::fs::remove_dir_all(&temp_dir).await?;
            }
        }
        Ok(())
    }
}

impl Drop for TestEnvironment {
    fn drop(&mut self) {
        if let Some(temp_dir) = &self.temp_dir {
            if temp_dir.exists() {
                // 在 Drop 中使用 blocking_remove 因为不能在 Drop 中使用 async
                if let Err(e) = std::fs::remove_dir_all(temp_dir) {
                    eprintln!("Failed to cleanup test directory: {}", e);
                }
            }
        }
    }
}

/// 测试环境构建器
pub struct TestEnvironmentBuilder {
    config: TestConfig,
}

impl TestEnvironmentBuilder {
    pub fn new() -> Self {
        Self {
            config: TestConfig::default(),
        }
    }
    
    pub fn with_server_port(mut self, port: u16) -> Self {
        self.config.server_port = port;
        self
    }
    
    pub fn with_data_shards(mut self, shards: usize) -> Self {
        self.config.data_shards = shards;
        self
    }
    
    pub fn with_parity_shards(mut self, shards: usize) -> Self {
        self.config.parity_shards = shards;
        self
    }
    
    pub fn with_request_timeout(mut self, timeout_ms: u64) -> Self {
        self.config.request_timeout_ms = timeout_ms;
        self
    }
    
    pub fn with_wait_timeout(mut self, timeout_ms: u64) -> Self {
        self.config.wait_timeout_ms = timeout_ms;
        self
    }
    
    pub async fn build(self) -> Result<TestEnvironment> {
        let mut env = TestEnvironment::new(self.config)?;
        env.setup().await?;
        Ok(env)
    }
}

impl Default for TestEnvironmentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 便捷函数：快速创建测试环境
pub async fn create_test_environment() -> Result<TestEnvironment> {
    TestEnvironmentBuilder::new().build().await
}

/// 便捷函数：创建自定义配置的测试环境
pub async fn create_test_environment_with_config(config: TestConfig) -> Result<TestEnvironment> {
    TestEnvironment::new(config)?.setup().await?;
    Ok(TestEnvironment::new(config)?)
}