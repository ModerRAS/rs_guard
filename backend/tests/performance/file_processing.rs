//! 文件处理性能测试
//! 
//! 测试文件读取、写入、编码等操作的性能

use std::time::Instant;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

/// 测试文件写入性能
pub fn test_file_write_performance() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let test_file = temp_dir.path().join("test_file.txt");
    
    let start = Instant::now();
    
    // 写入测试数据
    let test_data = "A".repeat(1024 * 1024); // 1MB 数据
    fs::write(&test_file, test_data)?;
    
    let duration = start.elapsed();
    println!("文件写入性能: {:?}", duration);
    
    Ok(())
}

/// 测试文件读取性能
pub fn test_file_read_performance() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let test_file = temp_dir.path().join("test_file.txt");
    
    // 先写入测试数据
    let test_data = "A".repeat(1024 * 1024); // 1MB 数据
    fs::write(&test_file, test_data)?;
    
    let start = Instant::now();
    
    // 读取测试数据
    let _data = fs::read_to_string(&test_file)?;
    
    let duration = start.elapsed();
    println!("文件读取性能: {:?}", duration);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_performance() {
        let result = test_file_write_performance();
        assert!(result.is_ok());
    }

    #[test]
    fn test_read_performance() {
        let result = test_file_read_performance();
        assert!(result.is_ok());
    }
}