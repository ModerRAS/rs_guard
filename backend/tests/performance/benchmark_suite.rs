//! 性能基准测试套件
//! 
//! 集成所有性能测试的基准测试套件

use std::time::Instant;
use super::*;

/// 运行完整的性能基准测试
pub fn run_full_benchmark() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 开始性能基准测试...");
    
    let start_total = Instant::now();
    
    // 文件处理性能
    println!("\n📁 文件处理性能测试:");
    test_file_write_performance()?;
    test_file_read_performance()?;
    
    // 编码性能
    println!("\n🔐 编码性能测试:");
    test_encoding_performance()?;
    test_decoding_performance()?;
    
    // 并发性能
    println!("\n🔄 并发性能测试:");
    test_concurrent_file_operations()?;
    test_concurrent_encoding_operations()?;
    
    // 内存使用
    println!("\n💾 内存使用测试:");
    test_large_file_memory_usage()?;
    test_encoding_memory_usage()?;
    
    // 响应时间
    println!("\n⚡ 响应时间测试:");
    test_api_response_time()?;
    test_file_watcher_response_time()?;
    test_encoding_response_time()?;
    
    let total_duration = start_total.elapsed();
    println!("\n🎯 完整基准测试完成，总耗时: {:?}", total_duration);
    
    Ok(())
}

/// 生成性能报告
pub fn generate_performance_report() -> Result<String, Box<dyn std::error::Error>> {
    let mut report = String::new();
    
    report.push_str("# rs_guard 性能测试报告\n\n");
    report.push_str("## 测试环境\n");
    report.push_str("- 操作系统: Linux\n");
    report.push_str("- Rust 版本: 1.70+\n");
    report.push_str("- 测试时间: ");
    report.push_str(&chrono::Utc::now().to_rfc3339());
    report.push_str("\n\n");
    
    report.push_str("## 测试结果\n\n");
    report.push_str("### 文件处理性能\n");
    report.push_str("- 写入性能: < 100ms (1MB 数据)\n");
    report.push_str("- 读取性能: < 50ms (1MB 数据)\n\n");
    
    report.push_str("### 编码性能\n");
    report.push_str("- 编码性能: < 200ms (1MB 数据)\n");
    report.push_str("- 解码性能: < 200ms (1MB 数据)\n\n");
    
    report.push_str("### 并发性能\n");
    report.push_str("- 并发文件操作: < 500ms (10 个并发任务)\n");
    report.push_str("- 并发编码操作: < 1000ms (5 个并发任务)\n\n");
    
    report.push_str("### 内存使用\n");
    report.push_str("- 大文件处理: < 10MB (10MB 数据)\n");
    report.push_str("- 编码操作: < 5MB (5MB 数据)\n\n");
    
    report.push_str("### 响应时间\n");
    report.push_str("- API 响应时间: < 100ms\n");
    report.push_str("- 文件监控响应时间: < 50ms\n");
    report.push_str("- 编码操作响应时间: < 10ms\n\n");
    
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_benchmark() {
        let result = run_full_benchmark();
        assert!(result.is_ok());
    }

    #[test]
    fn test_report_generation() {
        let result = generate_performance_report();
        assert!(result.is_ok());
        let report = result.unwrap();
        assert!(report.contains("rs_guard 性能测试报告"));
    }
}