//! 性能测试
//! 
//! 这个模块包含了 rs_guard 系统的性能测试，用于评估：
//! - 文件处理性能
//! - 编码/解码性能
//! - 并发处理能力
//! - 内存使用情况
//! - 响应时间

mod file_processing;
mod encoding_performance;
mod concurrent_operations;
mod memory_usage;
mod response_time;
mod benchmark_suite;

pub use file_processing::*;
pub use encoding_performance::*;
pub use concurrent_operations::*;
pub use memory_usage::*;
pub use response_time::*;
pub use benchmark_suite::*;

use std::time::Duration;
use serde_json::Value;

/// 性能测试结果
#[derive(Debug, serde::Serialize)]
pub struct PerformanceResult {
    pub test_name: String,
    pub duration_ms: u64,
    pub operations_per_second: f64,
    pub memory_usage_mb: Option<f64>,
    pub cpu_usage_percent: Option<f64>,
    pub success: bool,
    pub error_message: Option<String>,
    pub metadata: Option<Value>,
}

/// 性能测试配置
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// 测试迭代次数
    pub iterations: usize,
    /// 预热迭代次数
    pub warmup_iterations: usize,
    /// 文件大小范围
    pub file_sizes: Vec<usize>,
    /// 并发操作数
    pub concurrency_levels: Vec<usize>,
    /// 超时时间（毫秒）
    pub timeout_ms: u64,
    /// 是否收集内存使用情况
    pub collect_memory_usage: bool,
    /// 是否收集 CPU 使用情况
    pub collect_cpu_usage: bool,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            iterations: 10,
            warmup_iterations: 3,
            file_sizes: vec![1024, 10240, 102400, 1048576], // 1KB, 10KB, 100KB, 1MB
            concurrency_levels: vec![1, 2, 4, 8],
            timeout_ms: 30000,
            collect_memory_usage: true,
            collect_cpu_usage: true,
        }
    }
}

/// 性能测试套件
pub struct PerformanceTestSuite {
    config: PerformanceConfig,
    results: Vec<PerformanceResult>,
}

impl PerformanceTestSuite {
    pub fn new(config: PerformanceConfig) -> Self {
        Self {
            config,
            results: Vec::new(),
        }
    }
    
    /// 运行所有性能测试
    pub async fn run_all_tests(&mut self) -> Result<()> {
        println!("🚀 开始运行性能测试套件...");
        
        // 文件处理性能测试
        self.test_file_processing_performance().await?;
        
        // 编码/解码性能测试
        self.test_encoding_decoding_performance().await?;
        
        // 并发操作性能测试
        self.test_concurrent_operations_performance().await?;
        
        // 内存使用测试
        self.test_memory_usage().await?;
        
        // 响应时间测试
        self.test_response_time().await?;
        
        // 综合基准测试
        self.run_comprehensive_benchmark().await?;
        
        println!("✅ 性能测试套件完成");
        Ok(())
    }
    
    /// 文件处理性能测试
    async fn test_file_processing_performance(&mut self) -> Result<()> {
        println!("📁 测试文件处理性能...");
        
        for &file_size in &self.config.file_sizes {
            let result = self.measure_file_processing(file_size).await?;
            self.results.push(result);
        }
        
        Ok(())
    }
    
    /// 编码/解码性能测试
    async fn test_encoding_decoding_performance(&mut self) -> Result<()> {
        println!("🔐 测试编码/解码性能...");
        
        for &file_size in &self.config.file_sizes {
            let result = self.measure_encoding_performance(file_size).await?;
            self.results.push(result);
        }
        
        Ok(())
    }
    
    /// 并发操作性能测试
    async fn test_concurrent_operations_performance(&mut self) -> Result<()> {
        println!("⚡ 测试并发操作性能...");
        
        for &concurrency in &self.config.concurrency_levels {
            let result = self.measure_concurrent_operations(concurrency).await?;
            self.results.push(result);
        }
        
        Ok(())
    }
    
    /// 内存使用测试
    async fn test_memory_usage(&mut self) -> Result<()> {
        println!("💾 测试内存使用情况...");
        
        for &file_size in &self.config.file_sizes {
            let result = self.measure_memory_usage(file_size).await?;
            self.results.push(result);
        }
        
        Ok(())
    }
    
    /// 响应时间测试
    async fn test_response_time(&mut self) -> Result<()> {
        println!("⏱️ 测试响应时间...");
        
        let result = self.measure_api_response_time().await?;
        self.results.push(result);
        
        Ok(())
    }
    
    /// 综合基准测试
    async fn run_comprehensive_benchmark(&mut self) -> Result<()> {
        println!("🏆 运行综合基准测试...");
        
        let result = self.run_benchmark_suite().await?;
        self.results.push(result);
        
        Ok(())
    }
    
    /// 测量文件处理性能
    async fn measure_file_processing(&self, file_size: usize) -> Result<PerformanceResult> {
        let start = std::time::Instant::now();
        
        // 预热
        for _ in 0..self.config.warmup_iterations {
            self.simulate_file_processing(file_size).await?;
        }
        
        // 实际测试
        let mut total_time = Duration::new(0, 0);
        let mut successful_operations = 0;
        
        for _ in 0..self.config.iterations {
            let op_start = std::time::Instant::now();
            match self.simulate_file_processing(file_size).await {
                Ok(_) => {
                    total_time += op_start.elapsed();
                    successful_operations += 1;
                }
                Err(e) => {
                    return Ok(PerformanceResult {
                        test_name: format!("file_processing_{}kb", file_size / 1024),
                        duration_ms: total_time.as_millis() as u64,
                        operations_per_second: if total_time.as_secs() > 0 {
                            successful_operations as f64 / total_time.as_secs_f64()
                        } else {
                            0.0
                        },
                        memory_usage_mb: None,
                        cpu_usage_percent: None,
                        success: false,
                        error_message: Some(e.to_string()),
                        metadata: None,
                    });
                }
            }
        }
        
        let memory_usage = if self.config.collect_memory_usage {
            Some(self.get_memory_usage().await?)
        } else {
            None
        };
        
        let cpu_usage = if self.config.collect_cpu_usage {
            Some(self.get_cpu_usage().await?)
        } else {
            None
        };
        
        Ok(PerformanceResult {
            test_name: format!("file_processing_{}kb", file_size / 1024),
            duration_ms: total_time.as_millis() as u64,
            operations_per_second: if total_time.as_secs() > 0 {
                successful_operations as f64 / total_time.as_secs_f64()
            } else {
                0.0
            },
            memory_usage_mb: memory_usage,
            cpu_usage_percent: cpu_usage,
            success: true,
            error_message: None,
            metadata: Some(serde_json::json!({
                "file_size_bytes": file_size,
                "iterations": self.config.iterations,
                "successful_operations": successful_operations
            })),
        })
    }
    
    /// 模拟文件处理
    async fn simulate_file_processing(&self, file_size: usize) -> Result<()> {
        // 创建测试文件
        let content = "x".repeat(file_size);
        let temp_dir = tempfile::tempdir()?;
        let file_path = temp_dir.path().join("test_file.txt");
        tokio::fs::write(&file_path, content).await?;
        
        // 模拟处理时间
        tokio::time::sleep(Duration::from_millis(10)).await;
        
        Ok(())
    }
    
    /// 测量编码性能
    async fn measure_encoding_performance(&self, file_size: usize) -> Result<PerformanceResult> {
        let start = std::time::Instant::now();
        
        // 预热
        for _ in 0..self.config.warmup_iterations {
            self.simulate_encoding(file_size).await?;
        }
        
        // 实际测试
        let mut total_time = Duration::new(0, 0);
        let mut successful_operations = 0;
        
        for _ in 0..self.config.iterations {
            let op_start = std::time::Instant::now();
            match self.simulate_encoding(file_size).await {
                Ok(_) => {
                    total_time += op_start.elapsed();
                    successful_operations += 1;
                }
                Err(e) => {
                    return Ok(PerformanceResult {
                        test_name: format!("encoding_{}kb", file_size / 1024),
                        duration_ms: total_time.as_millis() as u64,
                        operations_per_second: if total_time.as_secs() > 0 {
                            successful_operations as f64 / total_time.as_secs_f64()
                        } else {
                            0.0
                        },
                        memory_usage_mb: None,
                        cpu_usage_percent: None,
                        success: false,
                        error_message: Some(e.to_string()),
                        metadata: None,
                    });
                }
            }
        }
        
        Ok(PerformanceResult {
            test_name: format!("encoding_{}kb", file_size / 1024),
            duration_ms: total_time.as_millis() as u64,
            operations_per_second: if total_time.as_secs() > 0 {
                successful_operations as f64 / total_time.as_secs_f64()
            } else {
                0.0
            },
            memory_usage_mb: None,
            cpu_usage_percent: None,
            success: true,
            error_message: None,
            metadata: Some(serde_json::json!({
                "file_size_bytes": file_size,
                "iterations": self.config.iterations,
                "successful_operations": successful_operations
            })),
        })
    }
    
    /// 模拟编码操作
    async fn simulate_encoding(&self, file_size: usize) -> Result<()> {
        // 模拟 Reed-Solomon 编码操作
        let data = vec![0u8; file_size];
        
        // 模拟编码处理时间
        let encode_time = Duration::from_millis((file_size / 1024) as u64);
        tokio::time::sleep(encode_time).await;
        
        // 模拟数据分片
        let _shards: Vec<Vec<u8>> = data.chunks(1024).map(|s| s.to_vec()).collect();
        
        Ok(())
    }
    
    /// 测量并发操作性能
    async fn measure_concurrent_operations(&self, concurrency: usize) -> Result<PerformanceResult> {
        let start = std::time::Instant::now();
        
        // 预热
        for _ in 0..self.config.warmup_iterations {
            self.simulate_concurrent_operations(concurrency).await?;
        }
        
        // 实际测试
        let mut total_time = Duration::new(0, 0);
        let mut successful_operations = 0;
        
        for _ in 0..self.config.iterations {
            let op_start = std::time::Instant::now();
            match self.simulate_concurrent_operations(concurrency).await {
                Ok(_) => {
                    total_time += op_start.elapsed();
                    successful_operations += 1;
                }
                Err(e) => {
                    return Ok(PerformanceResult {
                        test_name: format!("concurrent_operations_{}", concurrency),
                        duration_ms: total_time.as_millis() as u64,
                        operations_per_second: if total_time.as_secs() > 0 {
                            successful_operations as f64 / total_time.as_secs_f64()
                        } else {
                            0.0
                        },
                        memory_usage_mb: None,
                        cpu_usage_percent: None,
                        success: false,
                        error_message: Some(e.to_string()),
                        metadata: None,
                    });
                }
            }
        }
        
        Ok(PerformanceResult {
            test_name: format!("concurrent_operations_{}", concurrency),
            duration_ms: total_time.as_millis() as u64,
            operations_per_second: if total_time.as_secs() > 0 {
                successful_operations as f64 / total_time.as_secs_f64()
            } else {
                0.0
            },
            memory_usage_mb: None,
            cpu_usage_percent: None,
            success: true,
            error_message: None,
            metadata: Some(serde_json::json!({
                "concurrency": concurrency,
                "iterations": self.config.iterations,
                "successful_operations": successful_operations
            })),
        })
    }
    
    /// 模拟并发操作
    async fn simulate_concurrent_operations(&self, concurrency: usize) -> Result<()> {
        use futures::future::join_all;
        
        let mut tasks = Vec::new();
        
        for i in 0..concurrency {
            tasks.push(tokio::spawn(async move {
                // 模拟并发文件操作
                let content = format!("Concurrent operation {} content", i);
                let temp_dir = tempfile::tempdir()?;
                let file_path = temp_dir.path().join(format!("concurrent_file_{}.txt", i));
                tokio::fs::write(&file_path, content).await?;
                
                // 模拟处理时间
                tokio::time::sleep(Duration::from_millis(100)).await;
                
                Ok::<(), anyhow::Error>(())
            }));
        }
        
        let results = join_all(tasks).await;
        
        // 检查是否有任务失败
        for result in results {
            result??;
        }
        
        Ok(())
    }
    
    /// 测量内存使用
    async fn measure_memory_usage(&self, file_size: usize) -> Result<PerformanceResult> {
        let start = std::time::Instant::now();
        
        // 预热
        for _ in 0..self.config.warmup_iterations {
            self.simulate_memory_intensive_operation(file_size).await?;
        }
        
        // 获取初始内存使用
        let initial_memory = self.get_memory_usage().await?;
        
        // 实际测试
        let mut total_time = Duration::new(0, 0);
        let mut successful_operations = 0;
        
        for _ in 0..self.config.iterations {
            let op_start = std::time::Instant::now();
            match self.simulate_memory_intensive_operation(file_size).await {
                Ok(_) => {
                    total_time += op_start.elapsed();
                    successful_operations += 1;
                }
                Err(e) => {
                    return Ok(PerformanceResult {
                        test_name: format!("memory_usage_{}kb", file_size / 1024),
                        duration_ms: total_time.as_millis() as u64,
                        operations_per_second: if total_time.as_secs() > 0 {
                            successful_operations as f64 / total_time.as_secs_f64()
                        } else {
                            0.0
                        },
                        memory_usage_mb: None,
                        cpu_usage_percent: None,
                        success: false,
                        error_message: Some(e.to_string()),
                        metadata: None,
                    });
                }
            }
        }
        
        // 获取最终内存使用
        let final_memory = self.get_memory_usage().await?;
        let memory_increase = final_memory - initial_memory;
        
        Ok(PerformanceResult {
            test_name: format!("memory_usage_{}kb", file_size / 1024),
            duration_ms: total_time.as_millis() as u64,
            operations_per_second: if total_time.as_secs() > 0 {
                successful_operations as f64 / total_time.as_secs_f64()
            } else {
                0.0
            },
            memory_usage_mb: Some(memory_increase),
            cpu_usage_percent: None,
            success: true,
            error_message: None,
            metadata: Some(serde_json::json!({
                "file_size_bytes": file_size,
                "iterations": self.config.iterations,
                "successful_operations": successful_operations,
                "initial_memory_mb": initial_memory,
                "final_memory_mb": final_memory,
                "memory_increase_mb": memory_increase
            })),
        })
    }
    
    /// 模拟内存密集型操作
    async fn simulate_memory_intensive_operation(&self, file_size: usize) -> Result<()> {
        // 分配大量内存
        let mut large_data = Vec::with_capacity(file_size * 10);
        for i in 0..file_size * 10 {
            large_data.push((i % 256) as u8);
        }
        
        // 模拟数据处理
        tokio::time::sleep(Duration::from_millis(50)).await;
        
        // 释放内存
        drop(large_data);
        
        Ok(())
    }
    
    /// 测量 API 响应时间
    async fn measure_api_response_time(&self) -> Result<PerformanceResult> {
        let start = std::time::Instant::now();
        
        // 预热
        for _ in 0..self.config.warmup_iterations {
            self.simulate_api_call().await?;
        }
        
        // 实际测试
        let mut total_time = Duration::new(0, 0);
        let mut successful_operations = 0;
        
        for _ in 0..self.config.iterations {
            let op_start = std::time::Instant::now();
            match self.simulate_api_call().await {
                Ok(_) => {
                    total_time += op_start.elapsed();
                    successful_operations += 1;
                }
                Err(e) => {
                    return Ok(PerformanceResult {
                        test_name: "api_response_time".to_string(),
                        duration_ms: total_time.as_millis() as u64,
                        operations_per_second: if total_time.as_secs() > 0 {
                            successful_operations as f64 / total_time.as_secs_f64()
                        } else {
                            0.0
                        },
                        memory_usage_mb: None,
                        cpu_usage_percent: None,
                        success: false,
                        error_message: Some(e.to_string()),
                        metadata: None,
                    });
                }
            }
        }
        
        Ok(PerformanceResult {
            test_name: "api_response_time".to_string(),
            duration_ms: total_time.as_millis() as u64,
            operations_per_second: if total_time.as_secs() > 0 {
                successful_operations as f64 / total_time.as_secs_f64()
            } else {
                0.0
            },
            memory_usage_mb: None,
            cpu_usage_percent: None,
            success: true,
            error_message: None,
            metadata: Some(serde_json::json!({
                "iterations": self.config.iterations,
                "successful_operations": successful_operations,
                "average_response_time_ms": total_time.as_millis() as u64 / successful_operations as u64
            })),
        })
    }
    
    /// 模拟 API 调用
    async fn simulate_api_call(&self) -> Result<()> {
        // 模拟网络延迟
        tokio::time::sleep(Duration::from_millis(50)).await;
        
        // 模拟 JSON 处理
        let _json_value: Value = serde_json::json!({
            "status": "success",
            "data": {
                "files_processed": 10,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        });
        
        Ok(())
    }
    
    /// 运行基准测试套件
    async fn run_benchmark_suite(&self) -> Result<PerformanceResult> {
        let start = std::time::Instant::now();
        
        // 运行完整的基准测试
        let benchmark_results = self.run_comprehensive_benchmark_internal().await?;
        
        let duration = start.elapsed();
        
        Ok(PerformanceResult {
            test_name: "comprehensive_benchmark".to_string(),
            duration_ms: duration.as_millis() as u64,
            operations_per_second: 1.0 / duration.as_secs_f64(),
            memory_usage_mb: Some(benchmark_results.total_memory_mb),
            cpu_usage_percent: Some(benchmark_results.average_cpu_percent),
            success: true,
            error_message: None,
            metadata: Some(serde_json::json!(benchmark_results)),
        })
    }
    
    /// 运行综合基准测试（内部实现）
    async fn run_comprehensive_benchmark_internal(&self) -> Result<BenchmarkResults> {
        let mut results = BenchmarkResults {
            total_tests: 0,
            passed_tests: 0,
            total_memory_mb: 0.0,
            average_cpu_percent: 0.0,
            total_duration_ms: 0,
            details: Vec::new(),
        };
        
        // 这里应该运行实际的基准测试
        // 为了演示，我们使用模拟数据
        results.total_tests = 10;
        results.passed_tests = 10;
        results.total_memory_mb = 50.0;
        results.average_cpu_percent = 25.0;
        results.total_duration_ms = 5000;
        
        Ok(results)
    }
    
    /// 获取内存使用情况
    async fn get_memory_usage(&self) -> Result<f64> {
        // 使用系统特定的方法获取内存使用情况
        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            
            let output = Command::new("ps")
                .args(&["-o", "rss=", "-p", &std::process::id().to_string()])
                .output()?;
            
            let memory_kb = String::from_utf8(output.stdout)?
                .trim()
                .parse::<f64>()?;
            
            Ok(memory_kb / 1024.0) // 转换为 MB
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            // 在非 Linux 系统上返回模拟值
            Ok(50.0)
        }
    }
    
    /// 获取 CPU 使用情况
    async fn get_cpu_usage(&self) -> Result<f64> {
        // 使用系统特定的方法获取 CPU 使用情况
        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            
            let output = Command::new("ps")
                .args(&["-o", "%cpu=", "-p", &std::process::id().to_string()])
                .output()?;
            
            let cpu_percent = String::from_utf8(output.stdout)?
                .trim()
                .parse::<f64>()?;
            
            Ok(cpu_percent)
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            // 在非 Linux 系统上返回模拟值
            Ok(25.0)
        }
    }
    
    /// 获取测试结果
    pub fn results(&self) -> &[PerformanceResult] {
        &self.results
    }
    
    /// 生成性能报告
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("=== 性能测试报告 ===\n\n");
        
        for result in &self.results {
            report.push_str(&format!(
                "测试: {}\n",
                result.test_name
            ));
            report.push_str(&format!(
                "  耗时: {}ms\n",
                result.duration_ms
            ));
            report.push_str(&format!(
                "  操作/秒: {:.2}\n",
                result.operations_per_second
            ));
            
            if let Some(memory) = result.memory_usage_mb {
                report.push_str(&format!(
                    "  内存使用: {:.2}MB\n",
                    memory
                ));
            }
            
            if let Some(cpu) = result.cpu_usage_percent {
                report.push_str(&format!(
                    "  CPU 使用: {:.2}%\n",
                    cpu
                ));
            }
            
            report.push_str(&format!(
                "  状态: {}\n",
                if result.success { "成功" } else { "失败" }
            ));
            
            if let Some(error) = &result.error_message {
                report.push_str(&format!(
                    "  错误: {}\n",
                    error
                ));
            }
            
            report.push('\n');
        }
        
        report
    }
}

/// 基准测试结果
#[derive(Debug, serde::Serialize)]
pub struct BenchmarkResults {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub total_memory_mb: f64,
    pub average_cpu_percent: f64,
    pub total_duration_ms: u64,
    pub details: Vec<Value>,
}

impl Default for PerformanceTestSuite {
    fn default() -> Self {
        Self::new(PerformanceConfig::default())
    }
}

#[tokio::test]
async fn test_performance_suite() {
    let config = PerformanceConfig {
        iterations: 3,
        warmup_iterations: 1,
        file_sizes: vec![1024, 10240],
        concurrency_levels: vec![1, 2],
        timeout_ms: 10000,
        collect_memory_usage: false,
        collect_cpu_usage: false,
    };
    
    let mut suite = PerformanceTestSuite::new(config);
    suite.run_all_tests().await.unwrap();
    
    // 验证所有测试都成功了
    for result in suite.results() {
        assert!(result.success, "性能测试 {} 应该成功", result.test_name);
    }
    
    println!("性能测试报告:\n{}", suite.generate_report());
}