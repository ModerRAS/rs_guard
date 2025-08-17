//! 公共测试工具和辅助函数
//! 
//! 这个模块提供了所有测试共享的工具函数和辅助方法，
//! 包括测试环境管理、数据生成、HTTP 客户端等。

mod test_environment;
mod data_generator;
mod http_client;
mod assertions;
mod utils;
mod mock_server;
mod report_generator;

pub use test_environment::*;
pub use data_generator::*;
pub use http_client::*;
pub use assertions::*;
pub use utils::*;
pub use mock_server::*;
pub use report_generator::*;

use std::sync::Once;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// 全局初始化标志
static INIT: Once = Once::new();

/// 初始化测试环境
pub fn init_test_environment() {
    INIT.call_once(|| {
        // 初始化日志
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "debug".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
        
        // 设置测试环境变量
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("RUST_BACKTRACE", "1");
    });
}

/// 测试结果类型
pub type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

/// 测试配置
#[derive(Debug, Clone)]
pub struct TestConfig {
    /// 测试数据目录
    pub test_data_dir: String,
    /// 服务器端口（0 表示随机）
    pub server_port: u16,
    /// 数据分片数
    pub data_shards: usize,
    /// 校验分片数
    pub parity_shards: usize,
    /// 请求超时时间（毫秒）
    pub request_timeout_ms: u64,
    /// 等待超时时间（毫秒）
    pub wait_timeout_ms: u64,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            test_data_dir: "./test-data".to_string(),
            server_port: 0,
            data_shards: 4,
            parity_shards: 2,
            request_timeout_ms: 5000,
            wait_timeout_ms: 10000,
        }
    }
}

/// 测试辅助宏
#[macro_export]
macro_rules! assert_contains {
    ($haystack:expr, $needle:expr) => {
        assert!(
            $haystack.contains($needle),
            "Expected '{}' to contain '{}'",
            $haystack,
            $needle
        );
    };
}

#[macro_export]
macro_rules! assert_not_contains {
    ($haystack:expr, $needle:expr) => {
        assert!(
            !$haystack.contains($needle),
            "Expected '{}' not to contain '{}'",
            $haystack,
            $needle
        );
    };
}

#[macro_export]
macro_rules! test_timeout {
    ($timeout_ms:expr, $block:block) => {
        let timeout = std::time::Duration::from_millis($timeout_ms);
        let result = tokio::time::timeout(timeout, async move { $block }).await;
        
        match result {
            Ok(inner_result) => inner_result,
            Err(_) => panic!("Test timed out after {}ms", $timeout_ms),
        }
    };
}

#[macro_export]
macro_rules! retry {
    ($times:expr, $delay_ms:expr, $block:block) => {
        let mut result = None;
        for i in 0..$times {
            match $block {
                Ok(r) => {
                    result = Some(r);
                    break;
                }
                Err(e) => {
                    if i == $times - 1 {
                        return Err(e);
                    }
                    tokio::time::sleep(std::time::Duration::from_millis($delay_ms)).await;
                }
            }
        }
        result.unwrap()
    };
}

/// 测试用例 trait
pub trait TestCase {
    type Setup;
    type Input;
    type Output;
    
    fn name(&self) -> String;
    fn description(&self) -> String;
    
    async fn setup(&self) -> TestResult<Self::Setup>;
    async fn execute(&self, setup: &Self::Setup, input: Self::Input) -> TestResult<Self::Output>;
    async fn teardown(&self, setup: Self::Setup) -> TestResult<()>;
    
    async fn run(&self, input: Self::Input) -> TestResult<Self::Output> {
        let setup = self.setup().await?;
        let result = self.execute(&setup, input).await;
        self.teardown(setup).await?;
        result
    }
}

/// 测试套件 trait
pub trait TestSuite {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn setup(&self) -> TestResult<()>;
    fn teardown(&self) -> TestResult<()>;
    fn run_all(&self) -> TestResult<()>;
}

/// 测试报告结构
#[derive(Debug, serde::Serialize)]
pub struct TestReport {
    pub suite_name: String,
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub duration_ms: u64,
    pub test_results: Vec<TestResultEntry>,
}

#[derive(Debug, serde::Serialize)]
pub struct TestResultEntry {
    pub name: String,
    pub status: TestStatus,
    pub duration_ms: u64,
    pub error_message: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub enum TestStatus {
    Passed,
    Failed,
    Skipped,
}

/// 并发测试运行器
pub struct ConcurrentTestRunner {
    max_concurrency: usize,
}

impl ConcurrentTestRunner {
    pub fn new(max_concurrency: usize) -> Self {
        Self { max_concurrency }
    }
    
    pub async fn run_tests<F, Fut>(&self, tests: Vec<F>) -> Vec<TestResultEntry>
    where
        F: Fn() -> Fut + Send + Sync,
        Fut: std::future::Future<Output = TestResult<()>> + Send,
    {
        use futures::stream::{self, StreamExt};
        
        let results = stream::iter(tests)
            .buffer_unordered(self.max_concurrency)
            .collect::<Vec<_>>()
            .await;
        
        results
            .into_iter()
            .map(|result| match result {
                Ok(_) => TestResultEntry {
                    name: String::new(), // 名称需要外部设置
                    status: TestStatus::Passed,
                    duration_ms: 0,
                    error_message: None,
                },
                Err(e) => TestResultEntry {
                    name: String::new(),
                    status: TestStatus::Failed,
                    duration_ms: 0,
                    error_message: Some(e.to_string()),
                },
            })
            .collect()
    }
}

impl Default for ConcurrentTestRunner {
    fn default() -> Self {
        Self::new(4)
    }
}

/// 性能测试工具
pub struct PerformanceTestUtils;

impl PerformanceTestUtils {
    /// 测量函数执行时间
    pub async fn measure_time<F, Fut>(func: F) -> (Fut::Output, std::time::Duration)
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future,
    {
        let start = std::time::Instant::now();
        let result = func().await;
        let duration = start.elapsed();
        (result, duration)
    }
    
    /// 运行基准测试
    pub async fn run_benchmark<F, Fut>(
        name: &str,
        iterations: usize,
        func: F,
    ) -> BenchmarkResult
    where
        F: Fn() -> Fut + Clone,
        Fut: std::future::Future<Output = ()>,
    {
        let mut durations = Vec::with_capacity(iterations);
        
        for _ in 0..iterations {
            let (_, duration) = Self::measure_time(|| func()).await;
            durations.push(duration);
        }
        
        BenchmarkResult::new(name, durations)
    }
}

/// 基准测试结果
#[derive(Debug)]
pub struct BenchmarkResult {
    pub name: String,
    pub iterations: usize,
    pub total_duration: std::time::Duration,
    pub average_duration: std::time::Duration,
    pub min_duration: std::time::Duration,
    pub max_duration: std::time::Duration,
    pub median_duration: std::time::Duration,
}

impl BenchmarkResult {
    fn new(name: &str, durations: Vec<std::time::Duration>) -> Self {
        let iterations = durations.len();
        let total_duration: std::time::Duration = durations.iter().sum();
        let average_duration = total_duration / iterations as u32;
        let min_duration = durations.iter().min().copied().unwrap_or_default();
        let max_duration = durations.iter().max().copied().unwrap_or_default();
        
        // 计算中位数
        let mut sorted_durations = durations.clone();
        sorted_durations.sort();
        let median_duration = if sorted_durations.is_empty() {
            std::time::Duration::default()
        } else if sorted_durations.len() % 2 == 0 {
            let mid = sorted_durations.len() / 2;
            (sorted_durations[mid - 1] + sorted_durations[mid]) / 2
        } else {
            sorted_durations[sorted_durations.len() / 2]
        };
        
        Self {
            name: name.to_string(),
            iterations,
            total_duration,
            average_duration,
            min_duration,
            max_duration,
            median_duration,
        }
    }
    
    pub fn summary(&self) -> String {
        format!(
            "Benchmark: {}\n\
             Iterations: {}\n\
             Total: {:.2}s\n\
             Average: {:.2}ms\n\
             Min: {:.2}ms\n\
             Max: {:.2}ms\n\
             Median: {:.2}ms",
            self.name,
            self.iterations,
            self.total_duration.as_secs_f64(),
            self.average_duration.as_millis(),
            self.min_duration.as_millis(),
            self.max_duration.as_millis(),
            self.median_duration.as_millis()
        )
    }
}