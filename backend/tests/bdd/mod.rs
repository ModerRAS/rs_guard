//! BDD (Behavior Driven Development) 测试框架
//! 
//! 这个模块提供了基于 Cucumber 的 BDD 测试框架，支持：
//! - Gherkin 语法特性文件
//! - 步骤定义
//! - 测试世界状态管理
//! - 测试环境管理

mod steps;
mod world;
mod utils;

pub use steps::*;
pub use world::*;
pub use utils::*;

use cucumber::{given, then, when, World, WorldInit};

/// 重新导出 BDD 相关的宏和类型
pub use cucumber::{gherkin, runner, writer};

/// BDD 测试配置
#[derive(Debug, Clone)]
pub struct BddConfig {
    /// 测试特性文件目录
    pub features_dir: String,
    /// 最大并发场景数
    pub max_concurrent_scenarios: usize,
    /// 输出格式
    pub output_format: OutputFormat,
    /// 详细级别
    pub verbosity: Verbosity,
}

impl Default for BddConfig {
    fn default() -> Self {
        Self {
            features_dir: "tests/bdd/features".to_string(),
            max_concurrent_scenarios: 1,
            output_format: OutputFormat::Pretty,
            verbosity: Verbosity::Normal,
        }
    }
}

/// 输出格式
#[derive(Debug, Clone)]
pub enum OutputFormat {
    Pretty,
    Json,
    JUnit,
}

/// 详细级别
#[derive(Debug, Clone)]
pub enum Verbosity {
    Quiet,
    Normal,
    Verbose,
}

/// BDD 测试运行器
pub struct BddRunner {
    config: BddConfig,
}

impl BddRunner {
    pub fn new() -> Self {
        Self {
            config: BddConfig::default(),
        }
    }

    pub fn with_config(config: BddConfig) -> Self {
        Self { config }
    }

    /// 运行 BDD 测试
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut runner = RsGuardWorld::cucumber();

        // 配置并发性
        runner = runner.with_runner(cucumber::runner::Runner::new()
            .max_concurrent_scenarios(self.config.max_concurrent_scenarios));

        // 配置输出格式
        match self.config.output_format {
            OutputFormat::Pretty => {
                // 默认的漂亮输出
            }
            OutputFormat::Json => {
                runner = runner.with_writer(cucumber::writer::Json::new(
                    std::io::stdout(),
                ));
            }
            OutputFormat::JUnit => {
                runner = runner.with_writer(cucumber::writer::JUnit::new(
                    std::io::stdout(),
                ));
            }
        }

        // 运行测试
        runner.run_and_exit(&self.config.features_dir).await;
        
        Ok(())
    }

    /// 运行单个特性文件
    pub async fn run_feature(self, feature_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut runner = RsGuardWorld::cucumber();
        
        runner = runner.with_runner(cucumber::runner::Runner::new()
            .max_concurrent_scenarios(self.config.max_concurrent_scenarios));

        runner.run_and_exit(feature_file).await;
        
        Ok(())
    }
}

impl Default for BddRunner {
    fn default() -> Self {
        Self::new()
    }
}

/// 辅助宏：定义 BDD 测试场景
#[macro_export]
macro_rules! bdd_scenario {
    ($name:expr, $steps:block) => {
        #[tokio::test]
        async fn $name() {
            $steps
        }
    };
}

/// 辅助宏：定义 BDD 测试特性
#[macro_export]
macro_rules! bdd_feature {
    ($name:expr, $description:expr, $scenarios:block) => {
        mod $name {
            use super::*;
            
            #[tokio::test]
            async fn feature_tests() {
                $scenarios
            }
        }
    };
}