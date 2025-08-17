# rs_guard 测试 API 规范

## 概述

本文档定义了 rs_guard 项目测试框架的 API 规范，包括测试工具、测试数据管理、模拟对象和测试报告生成的标准化接口。

## 测试框架 API

### 1. 测试环境管理 API

#### TestEnvironment 结构体

```rust
#[derive(Debug)]
pub struct TestEnvironment {
    pub temp_dir: TempDir,
    pub config: TestConfig,
    pub database: Arc<dyn Database>,
    pub file_system: Arc<dyn FileSystem>,
    pub network: Arc<dyn Network>,
}

impl TestEnvironment {
    /// 创建新的测试环境
    pub fn new() -> Result<Self, TestError>;
    
    /// 设置测试配置
    pub fn with_config(self, config: TestConfig) -> Self;
    
    /// 启动测试环境
    pub async fn start(&mut self) -> Result<(), TestError>;
    
    /// 停止测试环境
    pub async fn stop(&mut self) -> Result<(), TestError>;
    
    /// 清理测试环境
    pub async fn cleanup(self) -> Result<(), TestError>;
    
    /// 获取测试环境状态
    pub fn status(&self) -> TestEnvironmentStatus;
}
```

#### TestConfig 结构体

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    pub watched_directories: Vec<PathBuf>,
    pub data_shards: usize,
    pub parity_shards: usize,
    pub database_type: DatabaseType,
    pub log_level: LogLevel,
    pub timeout_seconds: u64,
    pub max_concurrent_operations: usize,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            watched_directories: vec![PathBuf::from("./test-data/source")],
            data_shards: 4,
            parity_shards: 2,
            database_type: DatabaseType::Memory,
            log_level: LogLevel::Debug,
            timeout_seconds: 30,
            max_concurrent_operations: 10,
        }
    }
}
```

### 2. 测试数据管理 API

#### TestDataFactory 特性

```rust
#[async_trait]
pub trait TestDataFactory: Send + Sync {
    /// 创建测试文件
    async fn create_test_file(&self, name: &str, size: usize) -> Result<PathBuf, TestError>;
    
    /// 创建测试目录结构
    async fn create_test_directory(&self, structure: &DirectoryStructure) -> Result<PathBuf, TestError>;
    
    /// 创建测试配置文件
    async fn create_config_file(&self, config: &Config) -> Result<PathBuf, TestError>;
    
    /// 创建测试数据库
    async fn create_test_database(&self, schema: &DatabaseSchema) -> Result<Arc<dyn Database>, TestError>;
    
    /// 生成随机测试数据
    fn generate_random_data(&self, size: usize) -> Vec<u8>;
    
    /// 生成确定性测试数据
    fn generate_deterministic_data(&self, seed: u64, size: usize) -> Vec<u8>;
    
    /// 清理测试数据
    async fn cleanup(&self) -> Result<(), TestError>;
}
```

#### DirectoryStructure 枚举

```rust
#[derive(Debug, Clone)]
pub enum DirectoryStructure {
    /// 简单平面结构
    Flat { file_count: usize, file_size: usize },
    
    /// 嵌套目录结构
    Nested { 
        depth: usize, 
        files_per_level: usize, 
        file_size: usize 
    },
    
    /// 自定义结构
    Custom { 
        structure: Vec<DirectoryNode> 
    },
}

#[derive(Debug, Clone)]
pub struct DirectoryNode {
    pub name: String,
    pub node_type: NodeType,
    pub children: Vec<DirectoryNode>,
}

#[derive(Debug, Clone)]
pub enum NodeType {
    Directory,
    File { size: usize },
}
```

### 3. 模拟对象 API

#### MockFileSystem 结构体

```rust
#[derive(Debug)]
pub struct MockFileSystem {
    files: HashMap<PathBuf, MockFile>,
    events: Vec<FileSystemEvent>,
    event_sender: tokio::sync::mpsc::UnboundedSender<FileSystemEvent>,
}

impl MockFileSystem {
    /// 创建新的模拟文件系统
    pub fn new() -> Self;
    
    /// 添加文件到模拟文件系统
    pub fn add_file(&mut self, path: &Path, content: Vec<u8>) -> Result<(), TestError>;
    
    /// 添加目录到模拟文件系统
    pub fn add_directory(&mut self, path: &Path) -> Result<(), TestError>;
    
    /// 删除文件
    pub fn remove_file(&mut self, path: &Path) -> Result<(), TestError>;
    
    /// 修改文件内容
    pub fn modify_file(&mut self, path: &Path, new_content: Vec<u8>) -> Result<(), TestError>;
    
    /// 模拟文件系统事件
    pub async fn simulate_event(&mut self, event: FileSystemEvent) -> Result<(), TestError>;
    
    /// 获取文件系统事件监听器
    pub fn event_listener(&self) -> tokio::sync::mpsc::UnboundedReceiver<FileSystemEvent>;
    
    /// 验证文件是否存在
    pub fn file_exists(&self, path: &Path) -> bool;
    
    /// 获取文件内容
    pub fn read_file(&self, path: &Path) -> Result<Vec<u8>, TestError>;
    
    /// 获取文件元数据
    pub fn metadata(&self, path: &Path) -> Result<MockFileMetadata, TestError>;
}

#[derive(Debug, Clone)]
pub struct MockFile {
    pub content: Vec<u8>,
    pub metadata: MockFileMetadata,
    pub created_at: SystemTime,
    pub modified_at: SystemTime,
}

#[derive(Debug, Clone)]
pub struct MockFileMetadata {
    pub size: u64,
    pub is_file: bool,
    pub is_directory: bool,
    pub permissions: u32,
}
```

#### MockDatabase 结构体

```rust
#[derive(Debug)]
pub struct MockDatabase {
    data: HashMap<String, Vec<u8>>,
    metadata: HashMap<String, DatabaseMetadata>,
    transaction_log: Vec<DatabaseTransaction>,
    delay_ms: u64,
    failure_rate: f64,
}

impl MockDatabase {
    /// 创建新的模拟数据库
    pub fn new() -> Self;
    
    /// 设置操作延迟（模拟网络延迟）
    pub fn with_delay(self, delay_ms: u64) -> Self;
    
    /// 设置失败率（模拟故障）
    pub fn with_failure_rate(self, failure_rate: f64) -> Self;
    
    /// 插入数据
    pub async fn insert(&mut self, key: &str, value: Vec<u8>) -> Result<(), DatabaseError>;
    
    /// 获取数据
    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, DatabaseError>;
    
    /// 删除数据
    pub async fn delete(&mut self, key: &str) -> Result<bool, DatabaseError>;
    
    /// 更新数据
    pub async fn update(&mut self, key: &str, value: Vec<u8>) -> Result<bool, DatabaseError>;
    
    /// 开始事务
    pub async fn begin_transaction(&mut self) -> TransactionId;
    
    /// 提交事务
    pub async fn commit_transaction(&mut self, transaction_id: TransactionId) -> Result<(), DatabaseError>;
    
    /// 回滚事务
    pub async fn rollback_transaction(&mut self, transaction_id: TransactionId) -> Result<(), DatabaseError>;
    
    /// 查询数据
    pub async fn query(&self, query: &DatabaseQuery) -> Result<Vec<DatabaseRecord>, DatabaseError>;
    
    /// 验证数据完整性
    pub fn verify_integrity(&self) -> Result<(), DatabaseError>;
    
    /// 获取数据库统计信息
    pub fn stats(&self) -> DatabaseStats;
}
```

### 4. 测试断言 API

#### TestAssert 结构体

```rust
pub struct TestAssert {
    pub context: String,
    pub results: Vec<AssertionResult>,
}

impl TestAssert {
    /// 创建新的测试断言
    pub fn new(context: &str) -> Self;
    
    /// 断言相等
    pub fn assert_eq<T: PartialEq + Debug>(self, actual: T, expected: T) -> Self;
    
    /// 断言不相等
    pub fn assert_ne<T: PartialEq + Debug>(self, actual: T, expected: T) -> Self;
    
    /// 断言为真
    pub fn assert_true(self, condition: bool, message: &str) -> Self;
    
    /// 断言为假
    pub fn assert_false(self, condition: bool, message: &str) -> Self;
    
    /// 断言包含
    pub fn assert_contains<T: PartialEq + Debug>(self, container: &[T], item: T) -> Self;
    
    /// 断言不包含
    pub fn assert_not_contains<T: PartialEq + Debug>(self, container: &[T], item: T) -> Self;
    
    /// 断言为空
    pub fn assert_empty<T>(self, container: &[T]) -> Self;
    
    /// 断言不为空
    pub fn assert_not_empty<T>(self, container: &[T]) -> Self;
    
    /// 断言错误
    pub fn assert_err<T, E>(self, result: Result<T, E>) -> Self;
    
    /// 断言成功
    pub fn assert_ok<T, E>(self, result: Result<T, E>) -> Self;
    
    /// 断言性能指标
    pub fn assert_performance(self, actual: f64, expected: f64, tolerance: f64) -> Self;
    
    /// 断言超时
    pub fn assert_timeout<F, T>(self, future: F, timeout: Duration) -> Self
    where
        F: Future<Output = T>;
    
    /// 获取断言结果
    pub fn results(&self) -> &[AssertionResult];
    
    /// 检查是否所有断言都通过
    pub fn all_passed(&self) -> bool;
    
    /// 获取失败的断言
    pub fn failed_assertions(&self) -> Vec<&AssertionResult>;
}

#[derive(Debug, Clone)]
pub struct AssertionResult {
    pub passed: bool,
    pub message: String,
    pub context: String,
    pub timestamp: SystemTime,
}
```

### 5. 性能测试 API

#### PerformanceBenchmark 结构体

```rust
#[derive(Debug)]
pub struct PerformanceBenchmark {
    pub name: String,
    pub iterations: usize,
    pub warmup_iterations: usize,
    pub measurement_time: Duration,
    pub sample_size: usize,
}

impl PerformanceBenchmark {
    /// 创建新的性能基准测试
    pub fn new(name: &str) -> Self;
    
    /// 设置迭代次数
    pub fn with_iterations(self, iterations: usize) -> Self;
    
    /// 设置预热迭代次数
    pub fn with_warmup(self, warmup_iterations: usize) -> Self;
    
    /// 设置测量时间
    pub fn with_measurement_time(self, measurement_time: Duration) -> Self;
    
    /// 设置样本大小
    pub fn with_sample_size(self, sample_size: usize) -> Self;
    
    /// 运行基准测试
    pub async fn run<F, Fut>(&self, benchmark_fn: F) -> Result<BenchmarkResult, TestError>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = ()>;
    
    /// 比较基准测试结果
    pub fn compare(&self, result1: &BenchmarkResult, result2: &BenchmarkResult) -> ComparisonResult;
}

#[derive(Debug, Clone, Serialize)]
pub struct BenchmarkResult {
    pub name: String,
    pub iterations: usize,
    pub total_time: Duration,
    pub average_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
    pub median_time: Duration,
    pub std_deviation: Duration,
    pub throughput: f64,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
}

#[derive(Debug, Clone)]
pub struct ComparisonResult {
    pub baseline: BenchmarkResult,
    pub current: BenchmarkResult,
    pub time_difference: Duration,
    pub time_difference_percent: f64,
    pub throughput_difference: f64,
    pub memory_difference_mb: f64,
    pub cpu_difference_percent: f64,
    pub is_regression: bool,
    pub is_improvement: bool,
}
```

#### LoadTest 结构体

```rust
#[derive(Debug)]
pub struct LoadTest {
    pub name: String,
    pub duration: Duration,
    pub concurrent_users: usize,
    pub ramp_up_time: Duration,
    pub scenario: LoadTestScenario,
}

impl LoadTest {
    /// 创建新的负载测试
    pub fn new(name: &str) -> Self;
    
    /// 设置测试持续时间
    pub fn with_duration(self, duration: Duration) -> Self;
    
    /// 设置并发用户数
    pub fn with_concurrent_users(self, concurrent_users: usize) -> Self;
    
    /// 设置预热时间
    pub fn with_ramp_up_time(self, ramp_up_time: Duration) -> Self;
    
    /// 设置测试场景
    pub fn with_scenario(self, scenario: LoadTestScenario) -> Self;
    
    /// 运行负载测试
    pub async fn run(&self) -> Result<LoadTestResult, TestError>;
}

#[derive(Debug, Clone)]
pub enum LoadTestScenario {
    /// 文件创建场景
    FileCreation {
        file_size: usize,
        files_per_second: usize,
    },
    
    /// 编码操作场景
    EncodingOperations {
        file_size: usize,
        operations_per_second: usize,
    },
    
    /// 完整性检查场景
    IntegrityChecks {
        file_count: usize,
        checks_per_second: usize,
    },
    
    /// 混合场景
    Mixed {
        scenarios: Vec<(LoadTestScenario, f64)>, // (scenario, weight)
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct LoadTestResult {
    pub name: String,
    pub duration: Duration,
    pub concurrent_users: usize,
    pub total_requests: usize,
    pub successful_requests: usize,
    pub failed_requests: usize,
    pub requests_per_second: f64,
    pub average_response_time: Duration,
    pub min_response_time: Duration,
    pub max_response_time: Duration,
    pub p50_response_time: Duration,
    pub p90_response_time: Duration,
    pub p95_response_time: Duration,
    pub p99_response_time: Duration,
    pub error_rate: f64,
    pub throughput_mb_per_second: f64,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub network_throughput_mbps: f64,
}
```

### 6. BDD 测试 API

#### BDDTestRunner 结构体

```rust
#[derive(Debug)]
pub struct BDDTestRunner {
    pub feature_files: Vec<PathBuf>,
    pub step_definitions: Vec<StepDefinition>,
    pub world: BDDWorld,
    pub hooks: Vec<TestHook>,
}

impl BDDTestRunner {
    /// 创建新的 BDD 测试运行器
    pub fn new() -> Self;
    
    /// 添加特性文件
    pub fn add_feature_file(&mut self, path: &Path) -> Result<(), TestError>;
    
    /// 添加步骤定义
    pub fn add_step_definition(&mut self, step_def: StepDefinition);
    
    /// 添加测试钩子
    pub fn add_hook(&mut self, hook: TestHook);
    
    /// 运行所有测试
    pub async fn run_all(&self) -> Result<BDDTestReport, TestError>;
    
    /// 运行特定特性
    pub async fn run_feature(&self, feature_name: &str) -> Result<BDDTestReport, TestError>;
    
    /// 运行特定场景
    pub async fn run_scenario(&self, feature_name: &str, scenario_name: &str) -> Result<BDDTestReport, TestError>;
}

#[derive(Debug, Clone)]
pub struct StepDefinition {
    pub pattern: String,
    pub step_type: StepType,
    pub handler: Box<dyn StepHandler>,
}

#[derive(Debug, Clone)]
pub enum StepType {
    Given,
    When,
    Then,
    And,
    But,
}

#[async_trait]
pub trait StepHandler: Send + Sync {
    async fn execute(&self, world: &mut BDDWorld, args: Vec<String>) -> Result<(), TestError>;
}

#[derive(Debug, Clone)]
pub struct TestHook {
    pub hook_type: HookType,
    pub handler: Box<dyn HookHandler>,
}

#[derive(Debug, Clone)]
pub enum HookType {
    BeforeAll,
    AfterAll,
    BeforeFeature,
    AfterFeature,
    BeforeScenario,
    AfterScenario,
    BeforeStep,
    AfterStep,
}

#[async_trait]
pub trait HookHandler: Send + Sync {
    async fn execute(&self, world: &mut BDDWorld) -> Result<(), TestError>;
}

#[derive(Debug, Clone, Serialize)]
pub struct BDDTestReport {
    pub total_features: usize,
    pub passed_features: usize,
    pub failed_features: usize,
    pub total_scenarios: usize,
    pub passed_scenarios: usize,
    pub failed_scenarios: usize,
    pub total_steps: usize,
    pub passed_steps: usize,
    pub failed_steps: usize,
    pub skipped_steps: usize,
    pub duration: Duration,
    pub feature_results: Vec<FeatureResult>,
    pub errors: Vec<TestError>,
}
```

### 7. 测试报告 API

#### TestReportGenerator 结构体

```rust
#[derive(Debug)]
pub struct TestReportGenerator {
    pub config: ReportConfig,
    pub templates: HashMap<String, String>,
}

impl TestReportGenerator {
    /// 创建新的测试报告生成器
    pub fn new() -> Self;
    
    /// 设置报告配置
    pub fn with_config(self, config: ReportConfig) -> Self;
    
    /// 添加报告模板
    pub fn add_template(&mut self, name: &str, template: &str);
    
    /// 生成 HTML 报告
    pub async fn generate_html_report(&self, results: &TestResults) -> Result<String, TestError>;
    
    /// 生成 JSON 报告
    pub async fn generate_json_report(&self, results: &TestResults) -> Result<String, TestError>;
    
    /// 生成 XML 报告
    pub async fn generate_xml_report(&self, results: &TestResults) -> Result<String, TestError>;
    
    /// 生成 Markdown 报告
    pub async fn generate_markdown_report(&self, results: &TestResults) -> Result<String, TestError>;
    
    /// 生成控制台报告
    pub async fn generate_console_report(&self, results: &TestResults) -> Result<String, TestError>;
    
    /// 保存报告到文件
    pub async fn save_report(&self, results: &TestResults, path: &Path) -> Result<(), TestError>;
}

#[derive(Debug, Clone)]
pub struct ReportConfig {
    pub include_charts: bool,
    pub include_detailed_logs: bool,
    pub include_performance_metrics: bool,
    pub include_coverage_data: bool,
    pub theme: ReportTheme,
}

#[derive(Debug, Clone)]
pub enum ReportTheme {
    Light,
    Dark,
    Auto,
}

#[derive(Debug, Clone, Serialize)]
pub struct TestResults {
    pub test_run_id: String,
    pub timestamp: DateTime<Utc>,
    pub duration: Duration,
    pub environment: TestEnvironmentInfo,
    pub test_suites: Vec<TestSuiteResult>,
    pub summary: TestSummary,
    pub coverage: Option<TestCoverage>,
    pub performance: Option<TestPerformance>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TestSuiteResult {
    pub name: String,
    pub test_type: TestType,
    pub duration: Duration,
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub skipped_tests: usize,
    pub test_cases: Vec<TestCaseResult>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TestCaseResult {
    pub name: String,
    pub status: TestStatus,
    pub duration: Duration,
    pub error_message: Option<String>,
    pub error_stack_trace: Option<String>,
    pub performance_metrics: Option<TestPerformance>,
    pub attachments: Vec<TestAttachment>,
}
```

### 8. 测试工具 API

#### TestHelper 结构体

```rust
#[derive(Debug)]
pub struct TestHelper {
    pub environment: TestEnvironment,
    pub data_factory: Arc<dyn TestDataFactory>,
    pub assertions: TestAssert,
}

impl TestHelper {
    /// 创建新的测试助手
    pub fn new() -> Result<Self, TestError>;
    
    /// 等待条件满足
    pub async fn wait_for<F>(&self, condition: F, timeout: Duration) -> Result<(), TestError>
    where
        F: Fn() -> bool;
    
    /// 等待异步条件满足
    pub async fn wait_for_async<F, Fut>(&self, condition: F, timeout: Duration) -> Result<(), TestError>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = bool>;
    
    /// 重试操作
    pub async fn retry<F, Fut, T>(&self, operation: F, max_attempts: usize, delay: Duration) -> Result<T, TestError>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, TestError>>;
    
    /// 并发执行测试
    pub async fn run_concurrent<F, Fut>(&self, operations: Vec<F>, concurrency: usize) -> Result<Vec<F::Output>, TestError>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<F::Output, TestError>>;
    
    /// 测量执行时间
    pub async fn measure_time<F, Fut, T>(&self, operation: F) -> Result<(T, Duration), TestError>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, TestError>>;
    
    /// 模拟网络延迟
    pub async fn simulate_network_delay(&self, delay: Duration);
    
    /// 模拟系统负载
    pub async fn simulate_system_load(&self, cpu_percent: f64, memory_mb: f64, duration: Duration);
    
    /// 捕获日志输出
    pub async fn capture_logs<F, Fut>(&self, operation: F) -> Result<(F::Output, Vec<String>), TestError>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<F::Output, TestError>>;
}
```

## 使用示例

### 基本测试示例

```rust
#[tokio::test]
async fn test_file_monitoring_workflow() {
    // 创建测试环境
    let env = TestEnvironment::new().await.unwrap();
    
    // 创建测试助手
    let helper = TestHelper::new().unwrap();
    
    // 创建测试文件
    let test_file = helper.data_factory
        .create_test_file("test.txt", 1024)
        .await
        .unwrap();
    
    // 等待文件被监控
    helper.wait_for_async(|| async {
        let status = get_app_status().await;
        status.monitored_files > 0
    }, Duration::from_secs(5)).await.unwrap();
    
    // 验证文件被正确处理
    let processed = check_file_processed(&test_file).await;
    helper.assertions.assert_true(processed, "File should be processed");
    
    // 清理
    env.cleanup().await.unwrap();
}
```

### BDD 测试示例

```rust
#[tokio::test]
async fn test_data_integrity_bdd() {
    let mut runner = BDDTestRunner::new();
    
    // 添加步骤定义
    runner.add_step_definition(StepDefinition {
        pattern: r"系统正在监控目录 (.*)".to_string(),
        step_type: StepType::Given,
        handler: Box::new(GivenMonitoringDirectory::new()),
    });
    
    runner.add_step_definition(StepDefinition {
        pattern: r"在监控目录中创建文件 (.*)".to_string(),
        step_type: StepType::When,
        handler: Box::new(WhenCreateFile::new()),
    });
    
    runner.add_step_definition(StepDefinition {
        pattern: r"系统应该在 (\d+) 秒内检测到文件创建".to_string(),
        step_type: StepType::Then,
        handler: Box::new(ThenDetectFileCreation::new()),
    });
    
    // 运行测试
    let report = runner.run_feature("features/file_monitoring.feature").await.unwrap();
    
    // 验证结果
    assert!(report.passed_scenarios > 0);
}
```

### 性能测试示例

```rust
#[tokio::test]
async fn test_encoding_performance() {
    let benchmark = PerformanceBenchmark::new("encoding_performance")
        .with_iterations(100)
        .with_warmup(10)
        .with_measurement_time(Duration::from_secs(10))
        .with_sample_size(50);
    
    let result = benchmark.run(|| async {
        let data = vec![0u8; 1024 * 1024]; // 1MB
        encode_data(&data).await.unwrap();
    }).await.unwrap();
    
    // 断言性能指标
    assert!(result.average_time < Duration::from_millis(100));
    assert!(result.throughput > 10.0); // 10 MB/s
}
```

## 总结

本 API 规范提供了一个完整的、类型安全的测试框架接口，支持：

1. **测试环境管理**: 自动化的测试环境创建和清理
2. **测试数据管理**: 灵活的测试数据生成和管理
3. **模拟对象**: 完整的文件系统和数据库模拟
4. **测试断言**: 丰富的断言API和自定义断言
5. **性能测试**: 全面的性能基准和负载测试
6. **BDD 支持**: 完整的BDD测试框架
7. **测试报告**: 多格式的测试报告生成
8. **测试工具**: 实用的测试助手函数

通过这个API规范，可以构建一个强大、灵活且易于维护的测试框架，确保 rs_guard 项目的质量和可靠性。