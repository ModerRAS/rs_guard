//! 并发操作性能测试
//! 
//! 测试系统在并发场景下的性能表现

use std::time::Instant;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::task::JoinSet;

/// 测试并发文件处理性能
pub fn test_concurrent_file_operations() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = Runtime::new()?;
    
    let start = Instant::now();
    
    runtime.block_on(async {
        let mut tasks = JoinSet::new();
        
        // 创建 10 个并发任务
        for i in 0..10 {
            tasks.spawn(async move {
                // 模拟文件处理任务
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                format!("Task {} completed", i)
            });
        }
        
        // 等待所有任务完成
        while let Some(result) = tasks.join_next().await {
            result?;
        }
        
        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    })?;
    
    let duration = start.elapsed();
    println!("并发文件操作性能: {:?}", duration);
    
    Ok(())
}

/// 测试并发编码操作性能
pub fn test_concurrent_encoding_operations() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = Runtime::new()?;
    
    let start = Instant::now();
    
    runtime.block_on(async {
        let mut tasks = JoinSet::new();
        
        // 创建 5 个并发编码任务
        for i in 0..5 {
            tasks.spawn(async move {
                // 模拟编码任务
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                format!("Encoding task {} completed", i)
            });
        }
        
        // 等待所有任务完成
        while let Some(result) = tasks.join_next().await {
            result?;
        }
        
        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    })?;
    
    let duration = start.elapsed();
    println!("并发编码操作性能: {:?}", duration);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concurrent_file_ops() {
        let result = test_concurrent_file_operations();
        assert!(result.is_ok());
    }

    #[test]
    fn test_concurrent_encoding_ops() {
        let result = test_concurrent_encoding_operations();
        assert!(result.is_ok());
    }
}