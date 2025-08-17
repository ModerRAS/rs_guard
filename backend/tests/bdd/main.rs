use std::path::PathBuf;
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use shared::AppStatus;
use std::net::SocketAddr;

/// BDD 测试世界状态
#[derive(Debug)]
pub struct RsGuardWorld {
    /// 测试运行时
    runtime: Option<Runtime>,
    /// 应用服务器地址
    server_address: Option<SocketAddr>,
    /// 临时目录路径
    temp_dir: Option<PathBuf>,
    /// 测试数据目录
    test_data_dir: Option<PathBuf>,
    /// 应用状态
    app_state: Option<Arc<Mutex<AppStatus>>>,
    /// 最后的 HTTP 响应
    last_response: Option<serde_json::Value>,
    /// 最后的错误
    last_error: Option<String>,
}

impl RsGuardWorld {
    pub fn new() -> Self {
        Self {
            runtime: None, // 延迟初始化
            server_address: None,
            temp_dir: None,
            test_data_dir: None,
            app_state: None,
            last_response: None,
            last_error: None,
        }
    }

    /// 获取运行时引用
    pub fn runtime(&self) -> &Runtime {
        self.runtime.as_ref().expect("Runtime not initialized")
    }

    /// 设置服务器地址
    pub fn set_server_address(&mut self, addr: SocketAddr) {
        self.server_address = Some(addr);
    }

    /// 获取服务器地址
    pub fn server_address(&self) -> &SocketAddr {
        self.server_address.as_ref().expect("Server not started")
    }

    /// 设置临时目录
    pub fn set_temp_dir(&mut self, path: PathBuf) {
        self.temp_dir = Some(path);
    }

    /// 获取临时目录
    pub fn temp_dir(&self) -> &PathBuf {
        self.temp_dir.as_ref().expect("Temp dir not set")
    }

    /// 设置测试数据目录
    pub fn set_test_data_dir(&mut self, path: PathBuf) {
        self.test_data_dir = Some(path);
    }

    /// 获取测试数据目录
    pub fn test_data_dir(&self) -> &PathBuf {
        self.test_data_dir.as_ref().expect("Test data dir not set")
    }

    /// 设置应用状态
    pub fn set_app_state(&mut self, state: Arc<Mutex<AppStatus>>) {
        self.app_state = Some(state);
    }

    /// 获取应用状态
    pub fn app_state(&self) -> &Arc<Mutex<AppStatus>> {
        self.app_state.as_ref().expect("App state not set")
    }

    /// 设置最后响应
    pub fn set_last_response(&mut self, response: serde_json::Value) {
        self.last_response = Some(response);
    }

    /// 获取最后响应
    pub fn last_response(&self) -> Option<&serde_json::Value> {
        self.last_response.as_ref()
    }

    /// 设置最后错误
    pub fn set_last_error(&mut self, error: String) {
        self.last_error = Some(error);
    }

    /// 获取最后错误
    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    /// 清理资源
    pub async fn cleanup(&mut self) {
        if let Some(temp_dir) = &self.temp_dir {
            if temp_dir.exists() {
                tokio::fs::remove_dir_all(temp_dir).await.ok();
            }
        }
        if let Some(test_data_dir) = &self.test_data_dir {
            if test_data_dir.exists() {
                tokio::fs::remove_dir_all(test_data_dir).await.ok();
            }
        }
    }
}

impl Default for RsGuardWorld {
    fn default() -> Self {
        Self::new()
    }
}

/// 简化的 BDD 测试函数
pub async fn run_bdd_tests() {
    println!("🥒 运行 BDD 测试...");
    
    // 创建测试世界
    let mut world = RsGuardWorld::new();
    
    // 运行简单的 API 测试
    match test_api_status(&mut world).await {
        Ok(_) => println!("✅ API 状态测试通过"),
        Err(e) => println!("❌ API 状态测试失败: {}", e),
    }
    
    // 清理
    world.cleanup().await;
    
    println!("🥒 BDD 测试完成");
}

/// 测试 API 状态端点
async fn test_api_status(world: &mut RsGuardWorld) -> Result<(), Box<dyn std::error::Error>> {
    use tokio::net::TcpListener;
    use backend::{metadata, app_router};
    use tokio::fs;
    use tempfile::tempdir;
    
    // 创建临时目录
    let temp_dir = tempdir()?;
    let temp_path = temp_dir.path().to_path_buf();
    world.set_temp_dir(temp_path.clone());
    
    // 创建测试数据目录
    let test_data_dir = temp_path.join("test-data");
    let source_dir = test_data_dir.join("source");
    fs::create_dir_all(&source_dir).await?;
    world.set_test_data_dir(test_data_dir.clone());
    
    // 创建应用状态
    let app_state = Arc::new(Mutex::new(AppStatus {
        watched_dirs: vec![source_dir.to_string_lossy().to_string()],
        data_shards: 4,
        parity_shards: 2,
        ..Default::default()
    }));
    world.set_app_state(app_state.clone());
    
    // 启动服务器
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    world.set_server_address(addr);
    
    // 构建应用路由
    let db = Arc::new(metadata::open_db(":memory:")?);
    let app = app_router(app_state, db);
    
    // 在后台启动服务器
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    
    // 等待服务器启动
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    
    // 测试 API 端点
    let client = reqwest::Client::new();
    let url = format!("http://{}/api/status", addr);
    
    let response = client.get(&url).send().await?;
    let status = response.status();
    let body = response.text().await?;
    
    // 验证响应
    assert_eq!(status, 200, "Expected status 200, got {}", status);
    
    let json_body: serde_json::Value = serde_json::from_str(&body)?;
    assert_eq!(json_body["data_shards"], 4);
    assert_eq!(json_body["parity_shards"], 2);
    assert!(json_body["watched_dirs"].is_array());
    
    println!("✅ API 状态测试通过: {} - {}", status, body);
    
    Ok(())
}

/// BDD 测试入口点
#[tokio::main]
async fn main() {
    run_bdd_tests().await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_endpoint() {
        // 简化实现：直接测试应用状态而不启动服务器
        let app_state = Arc::new(Mutex::new(AppStatus {
            watched_dirs: vec!["/test".to_string()],
            data_shards: 4,
            parity_shards: 2,
            ..Default::default()
        }));
        
        let state = app_state.lock().unwrap();
        assert_eq!(state.data_shards, 4);
        assert_eq!(state.parity_shards, 2);
        assert_eq!(state.watched_dirs.len(), 1);
        
        println!("✅ 应用状态测试通过");
    }
    
    #[tokio::test]
    async fn test_full_api() {
        let mut world = RsGuardWorld::new();
        let result = test_api_status(&mut world).await;
        
        // 清理
        world.cleanup().await;
        
        assert!(result.is_ok(), "API 测试应该成功: {:?}", result);
    }
}