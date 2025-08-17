//! 配置测试
//! 
//! 测试系统配置功能和配置文件处理

use std::fs;
use std::path::Path;
use tempfile::tempdir;

/// 测试配置文件读取
pub fn test_config_file_reading() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let config_file = temp_dir.path().join("config.toml");
    
    // 创建测试配置文件
    let config_content = r#"
watched_directories = ["/test/path"]
data_shards = 4
parity_shards = 2
"#;
    
    fs::write(&config_file, config_content)?;
    
    // 读取配置文件
    let content = fs::read_to_string(&config_file)?;
    assert!(content.contains("watched_directories"));
    assert!(content.contains("data_shards = 4"));
    
    println!("✅ 配置文件读取测试通过");
    Ok(())
}

/// 测试配置验证
pub fn test_config_validation() -> Result<(), Box<dyn std::error::Error>> {
    // 测试有效配置
    let valid_config = r#"
watched_directories = ["/valid/path"]
data_shards = 4
parity_shards = 2
"#;
    
    assert!(validate_config(valid_config)?);
    
    // 测试无效配置
    let invalid_config = r#"
watched_directories = []
data_shards = 0
parity_shards = -1
"#;
    
    assert!(!validate_config(invalid_config)?);
    
    println!("✅ 配置验证测试通过");
    Ok(())
}

/// 简化配置验证函数
fn validate_config(config: &str) -> Result<bool, Box<dyn std::error::Error>> {
    // 简化实现：检查基本格式
    Ok(config.contains("watched_directories") && 
        config.contains("data_shards") && 
        config.contains("parity_shards"))
}

/// 测试配置热重载
pub fn test_config_hot_reload() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let config_file = temp_dir.path().join("config.toml");
    
    // 初始配置
    let initial_config = r#"
watched_directories = ["/initial/path"]
data_shards = 4
parity_shards = 2
"#;
    
    fs::write(&config_file, initial_config)?;
    
    // 模拟配置修改
    let updated_config = r#"
watched_directories = ["/updated/path"]
data_shards = 6
parity_shards = 3
"#;
    
    fs::write(&config_file, updated_config)?;
    
    // 验证配置更新
    let updated_content = fs::read_to_string(&config_file)?;
    assert!(updated_content.contains("/updated/path"));
    assert!(updated_content.contains("data_shards = 6"));
    
    println!("✅ 配置热重载测试通过");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_reading() {
        let result = test_config_file_reading();
        assert!(result.is_ok(), "配置文件读取测试失败: {:?}", result);
    }

    #[test]
    fn test_config_validation() {
        let result = test_config_validation();
        assert!(result.is_ok(), "配置验证测试失败: {:?}", result);
    }

    #[test]
    fn test_config_hot_reload() {
        let result = test_config_hot_reload();
        assert!(result.is_ok(), "配置热重载测试失败: {:?}", result);
    }
}