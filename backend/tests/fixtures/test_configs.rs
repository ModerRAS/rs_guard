//! 测试配置
//! 
//! 这个模块定义了各种测试用的配置。

/// 测试配置变体
pub struct TestConfigs;

impl TestConfigs {
    /// 基础配置 - 适合快速测试
    pub fn basic() -> backend::config::AppConfig {
        backend::config::AppConfig {
            watched_directories: vec!["./test-data/source".to_string()],
            data_shards: 4,
            parity_shards: 2,
        }
    }
    
    /// 最小配置 - 最小的分片配置
    pub fn minimal() -> backend::config::AppConfig {
        backend::config::AppConfig {
            watched_directories: vec!["./test-data/source".to_string()],
            data_shards: 2,
            parity_shards: 1,
        }
    }
    
    /// 标准配置 - 平衡性能和冗余
    pub fn standard() -> backend::config::AppConfig {
        backend::config::AppConfig {
            watched_directories: vec!["./test-data/source".to_string()],
            data_shards: 6,
            parity_shards: 3,
        }
    }
    
    /// 高冗余配置 - 更高的数据安全性
    pub fn high_redundancy() -> backend::config::AppConfig {
        backend::config::AppConfig {
            watched_directories: vec!["./test-data/source".to_string()],
            data_shards: 4,
            parity_shards: 4,
        }
    }
    
    /// 多目录配置 - 监控多个目录
    pub fn multi_directory() -> backend::config::AppConfig {
        backend::config::AppConfig {
            watched_directories: vec![
                "./test-data/source".to_string(),
                "./test-data/documents".to_string(),
                "./test-data/images".to_string(),
            ],
            data_shards: 4,
            parity_shards: 2,
        }
    }
    
    /// 性能测试配置 - 优化的性能设置
    pub fn performance() -> backend::config::AppConfig {
        backend::config::AppConfig {
            watched_directories: vec!["./test-data/source".to_string()],
            data_shards: 8,
            parity_shards: 2,
        }
    }
    
    /// 大文件配置 - 适合大文件处理
    pub fn large_files() -> backend::config::AppConfig {
        backend::config::AppConfig {
            watched_directories: vec!["./test-data/source".to_string()],
            data_shards: 16,
            parity_shards: 4,
        }
    }
    
    /// 无效配置 - 用于错误测试
    pub fn invalid() -> backend::config::AppConfig {
        backend::config::AppConfig {
            watched_directories: vec![], // 空目录列表
            data_shards: 0, // 无效的数据分片数
            parity_shards: 0, // 无效的校验分片数
        }
    }
    
    /// 极端配置 - 边界情况测试
    pub fn extreme() -> backend::config::AppConfig {
        backend::config::AppConfig {
            watched_directories: vec!["./test-data/source".to_string()],
            data_shards: 255, // 最大分片数
            parity_shards: 255, // 最大校验分片数
        }
    }
}

/// 测试环境配置
pub struct TestEnvConfigs;

impl TestEnvConfigs {
    /// 开发环境配置
    pub fn development() -> TestEnvConfig {
        TestEnvConfig {
            log_level: "debug".to_string(),
            request_timeout_ms: 30000,
            wait_timeout_ms: 10000,
            cleanup_on_exit: true,
            parallel_tests: true,
            max_concurrent: 4,
        }
    }
    
    /// CI/CD 环境配置
    pub fn ci_cd() -> TestEnvConfig {
        TestEnvConfig {
            log_level: "info".to_string(),
            request_timeout_ms: 60000,
            wait_timeout_ms: 30000,
            cleanup_on_exit: true,
            parallel_tests: false,
            max_concurrent: 1,
        }
    }
    
    /// 性能测试配置
    pub fn performance() -> TestEnvConfig {
        TestEnvConfig {
            log_level: "warn".to_string(),
            request_timeout_ms: 120000,
            wait_timeout_ms: 60000,
            cleanup_on_exit: false,
            parallel_tests: true,
            max_concurrent: 8,
        }
    }
    
    /// 调试配置
    pub fn debug() -> TestEnvConfig {
        TestEnvConfig {
            log_level: "trace".to_string(),
            request_timeout_ms: 0, // 无超时
            wait_timeout_ms: 0,   // 无超时
            cleanup_on_exit: false,
            parallel_tests: false,
            max_concurrent: 1,
        }
    }
}

/// 测试环境配置结构
#[derive(Debug, Clone)]
pub struct TestEnvConfig {
    pub log_level: String,
    pub request_timeout_ms: u64,
    pub wait_timeout_ms: u64,
    pub cleanup_on_exit: bool,
    pub parallel_tests: bool,
    pub max_concurrent: usize,
}

impl Default for TestEnvConfig {
    fn default() -> Self {
        Self {
            log_level: "info".to_string(),
            request_timeout_ms: 30000,
            wait_timeout_ms: 10000,
            cleanup_on_exit: true,
            parallel_tests: true,
            max_concurrent: 4,
        }
    }
}

/// BDD 测试配置
pub struct BddTestConfigs;

impl BddTestConfigs {
    /// 基础 BDD 配置
    pub fn basic() -> BddConfig {
        BddConfig {
            features_dir: "tests/bdd/features".to_string(),
            max_concurrent_scenarios: 1,
            output_format: OutputFormat::Pretty,
            verbosity: Verbosity::Normal,
        }
    }
    
    /// 详细 BDD 配置
    pub fn verbose() -> BddConfig {
        BddConfig {
            features_dir: "tests/bdd/features".to_string(),
            max_concurrent_scenarios: 1,
            output_format: OutputFormat::Pretty,
            verbosity: Verbosity::Verbose,
        }
    }
    
    /// CI/CD BDD 配置
    pub fn ci_cd() -> BddConfig {
        BddConfig {
            features_dir: "tests/bdd/features".to_string(),
            max_concurrent_scenarios: 1,
            output_format: OutputFormat::JUnit,
            verbosity: Verbosity::Normal,
        }
    }
}

/// BDD 配置结构
#[derive(Debug, Clone)]
pub struct BddConfig {
    pub features_dir: String,
    pub max_concurrent_scenarios: usize,
    pub output_format: OutputFormat,
    pub verbosity: Verbosity,
}

/// 输出格式枚举
#[derive(Debug, Clone)]
pub enum OutputFormat {
    Pretty,
    Json,
    JUnit,
}

/// 详细级别枚举
#[derive(Debug, Clone)]
pub enum Verbosity {
    Quiet,
    Normal,
    Verbose,
}

/// 性能测试配置
pub struct PerformanceTestConfigs;

impl PerformanceTestConfigs {
    /// 快速性能测试
    pub fn quick() -> PerformanceConfig {
        PerformanceConfig {
            iterations: 10,
            warmup_iterations: 3,
            file_sizes: vec![1024, 10240, 102400], // 1KB, 10KB, 100KB
            concurrent_operations: vec![1, 4, 8],
            timeout_ms: 30000,
        }
    }
    
    /// 标准性能测试
    pub fn standard() -> PerformanceConfig {
        PerformanceConfig {
            iterations: 50,
            warmup_iterations: 10,
            file_sizes: vec![1024, 10240, 102400, 1048576], // 1KB, 10KB, 100KB, 1MB
            concurrent_operations: vec![1, 2, 4, 8],
            timeout_ms: 60000,
        }
    }
    
    /// 全面性能测试
    pub fn comprehensive() -> PerformanceConfig {
        PerformanceConfig {
            iterations: 100,
            warmup_iterations: 20,
            file_sizes: vec![
                1024,       // 1KB
                10240,      // 10KB
                102400,     // 100KB
                1048576,    // 1MB
                10485760,   // 10MB
            ],
            concurrent_operations: vec![1, 2, 4, 8, 16],
            timeout_ms: 120000,
        }
    }
}

/// 性能测试配置结构
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    pub iterations: usize,
    pub warmup_iterations: usize,
    pub file_sizes: Vec<usize>,
    pub concurrent_operations: Vec<usize>,
    pub timeout_ms: u64,
}

/// 集成测试配置
pub struct IntegrationTestConfigs;

impl IntegrationTestConfigs {
    /// 基础集成测试配置
    pub fn basic() -> IntegrationConfig {
        IntegrationConfig {
            test_data_files: vec![
                ("basic.txt", "Basic test content"),
                ("medium.txt", &"Medium test content. ".repeat(100)),
                ("large.txt", &"Large test content. ".repeat(1000)),
            ],
            expected_file_count: 3,
            timeout_ms: 30000,
            cleanup_after_test: true,
        }
    }
    
    /// 复杂集成测试配置
    pub fn complex() -> IntegrationConfig {
        IntegrationConfig {
            test_data_files: vec![
                ("basic.txt".to_string(), "Basic test content".to_string()),
                ("json.json".to_string(), r#"{"type": "test", "version": 1}"#.to_string()),
                ("csv.csv".to_string(), "id,name,value\n1,test,100\n2,demo,200".to_string()),
                ("binary.bin".to_string(), "\x01\x02\x03\x04".to_string()),
                ("unicode.txt".to_string(), "Unicode: 中文 🚀".to_string()),
            ],
            expected_file_count: 5,
            timeout_ms: 60000,
            cleanup_after_test: true,
        }
    }
    
    /// 压力测试配置
    pub fn stress() -> IntegrationConfig {
        IntegrationConfig {
            test_data_files: (0..100)
                .map(|i| (format!("file_{}.txt", i), format!("Content for file {}", i)))
                .collect(),
            expected_file_count: 100,
            timeout_ms: 120000,
            cleanup_after_test: true,
        }
    }
}

/// 集成测试配置结构
#[derive(Debug, Clone)]
pub struct IntegrationConfig {
    pub test_data_files: Vec<(String, String)>,
    pub expected_file_count: usize,
    pub timeout_ms: u64,
    pub cleanup_after_test: bool,
}

/// 网络测试配置
pub struct NetworkTestConfigs;

impl NetworkTestConfigs {
    /// 基础网络测试配置
    pub fn basic() -> NetworkConfig {
        NetworkConfig {
            server_host: "127.0.0.1".to_string(),
            server_port: 0, // 随机端口
            client_timeout_ms: 5000,
            max_connections: 10,
            use_https: false,
        }
    }
    
    /// 高并发网络测试配置
    pub fn high_concurrency() -> NetworkConfig {
        NetworkConfig {
            server_host: "127.0.0.1".to_string(),
            server_port: 0,
            client_timeout_ms: 30000,
            max_connections: 100,
            use_https: false,
        }
    }
}

/// 网络测试配置结构
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub server_host: String,
    pub server_port: u16,
    pub client_timeout_ms: u64,
    pub max_connections: usize,
    pub use_https: bool,
}

/// 便捷宏：使用测试配置
#[macro_export]
macro_rules! with_test_config {
    ($config:expr, $block:block) => {{
        let original_config = std::env::var("RUST_LOG").ok();
        
        // 设置测试环境
        std::env::set_var("RUST_LOG", &$config.log_level);
        
        let result = $block;
        
        // 恢复原始配置
        match original_config {
            Some(config) => std::env::set_var("RUST_LOG", config),
            None => std::env::remove_var("RUST_LOG"),
        }
        
        result
    }};
}

/// 便捷宏：使用性能测试配置
#[macro_export]
macro_rules! with_performance_config {
    ($config:expr, $block:block) => {{
        let start = std::time::Instant::now();
        let result = $block;
        let duration = start.elapsed();
        
        if duration > std::time::Duration::from_millis($config.timeout_ms) {
            panic!("Performance test exceeded timeout of {}ms", $config.timeout_ms);
        }
        
        (result, duration)
    }};
}