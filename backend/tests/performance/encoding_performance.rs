//! 编码性能测试
//! 
//! 测试 Reed-Solomon 编码/解码的性能

use std::time::Instant;
use reed_solomon_erasure::ReedSolomon;

/// 测试编码性能
pub fn test_encoding_performance() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1u8; 1024 * 1024]; // 1MB 测试数据
    let r = ReedSolomon::new(4, 2)?; // 4 数据分片，2 校验分片
    
    let start = Instant::now();
    
    // 编码
    let _encoded = r.encode(&data)?;
    
    let duration = start.elapsed();
    println!("编码性能: {:?}", duration);
    
    Ok(())
}

/// 测试解码性能
pub fn test_decoding_performance() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1u8; 1024 * 1024]; // 1MB 测试数据
    let r = ReedSolomon::new(4, 2)?; // 4 数据分片，2 校验分片
    
    // 先编码
    let encoded = r.encode(&data)?;
    
    let start = Instant::now();
    
    // 解码
    let _decoded = r.reconstruct(&encoded)?;
    
    let duration = start.elapsed();
    println!("解码性能: {:?}", duration);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding_perf() {
        let result = test_encoding_performance();
        assert!(result.is_ok());
    }

    #[test]
    fn test_decoding_perf() {
        let result = test_decoding_performance();
        assert!(result.is_ok());
    }
}