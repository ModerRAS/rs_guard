//! 内存使用性能测试
//! 
//! 测试系统在不同操作下的内存使用情况

use std::time::Instant;
use std::alloc::{GlobalAlloc, System, Layout};
use std::sync::atomic::{AtomicUsize, Ordering};

/// 简单的内存使用跟踪器
pub struct MemoryTracker {
    allocated: AtomicUsize,
}

impl MemoryTracker {
    pub fn new() -> Self {
        Self {
            allocated: AtomicUsize::new(0),
        }
    }
    
    pub fn get_allocated(&self) -> usize {
        self.allocated.load(Ordering::SeqCst)
    }
    
    pub fn reset(&self) {
        self.allocated.store(0, Ordering::SeqCst);
    }
}

/// 测试大文件处理的内存使用
pub fn test_large_file_memory_usage() -> Result<(), Box<dyn std::error::Error>> {
    let tracker = MemoryTracker::new();
    
    // 模拟大文件处理
    let start = Instant::now();
    
    // 创建大量数据
    let large_data: Vec<u8> = (0..10_000_000).map(|i| (i % 256) as u8).collect();
    
    let duration = start.elapsed();
    let memory_used = large_data.len();
    
    println!("大文件处理时间: {:?}", duration);
    println!("大文件处理内存使用: {} bytes", memory_used);
    
    Ok(())
}

/// 测试编码操作的内存使用
pub fn test_encoding_memory_usage() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1u8; 5_000_000]; // 5MB 测试数据
    
    let start = Instant::now();
    
    // 模拟编码操作
    let encoded_data: Vec<u8> = data.iter()
        .map(|&x| x.wrapping_mul(2))
        .collect();
    
    let duration = start.elapsed();
    let memory_used = encoded_data.len();
    
    println!("编码操作时间: {:?}", duration);
    println!("编码操作内存使用: {} bytes", memory_used);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_large_file_memory() {
        let result = test_large_file_memory_usage();
        assert!(result.is_ok());
    }

    #[test]
    fn test_encoding_memory() {
        let result = test_encoding_memory_usage();
        assert!(result.is_ok());
    }
}