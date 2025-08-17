//! 测试数据和配置
//! 
//! 这个模块包含了各种测试用的固定数据、配置文件和测试场景。

mod test_data;
mod test_configs;
mod test_scenarios;

pub use test_data::*;
pub use test_configs::*;
pub use test_scenarios::*;

use std::path::PathBuf;

/// 测试文件路径
pub struct TestPaths;

impl TestPaths {
    /// 获取测试数据目录
    pub fn test_data_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join("data")
    }
    
    /// 获取测试配置目录
    pub fn test_config_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join("config")
    }
    
    /// 获取测试场景目录
    pub fn test_scenarios_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join("scenarios")
    }
    
    /// 获取测试文件路径
    pub fn test_file(filename: &str) -> PathBuf {
        Self::test_data_dir().join(filename)
    }
    
    /// 获取测试配置文件路径
    pub fn test_config_file(filename: &str) -> PathBuf {
        Self::test_config_dir().join(filename)
    }
    
    /// 获取测试场景文件路径
    pub fn test_scenario_file(filename: &str) -> PathBuf {
        Self::test_scenarios_dir().join(filename)
    }
}

/// 测试数据管理器
pub struct TestDataManager {
    base_dir: PathBuf,
}

impl TestDataManager {
    pub fn new() -> Self {
        Self {
            base_dir: TestPaths::test_data_dir(),
        }
    }
    
    /// 创建所有测试数据
    pub async fn create_all_test_data(&self) -> Result<()> {
        self.create_basic_test_files().await?;
        self.create_large_test_files().await?;
        self.create_binary_test_files().await?;
        self.create_special_test_files().await?;
        self.create_directory_structures().await?;
        Ok(())
    }
    
    /// 创建基础测试文件
    async fn create_basic_test_files(&self) -> Result<()> {
        use tokio::fs;
        
        let data_dir = self.base_dir.join("basic");
        fs::create_dir_all(&data_dir).await?;
        
        // 创建各种大小的文本文件
        let files = vec![
            ("small.txt", "Small file content"),
            ("medium.txt", &"Medium file content. ".repeat(100)),
            ("large.txt", &"Large file content. ".repeat(1000)),
            ("empty.txt", ""),
            ("unicode.txt", "Unicode content: 中文 🚀 emojis 😊"),
            ("special_chars.txt", "Special chars: !@#$%^&*()_+-=[]{}|;':\",./<>?"),
        ];
        
        for (filename, content) in files {
            let file_path = data_dir.join(filename);
            fs::write(&file_path, content).await?;
        }
        
        Ok(())
    }
    
    /// 创建大型测试文件
    async fn create_large_test_files(&self) -> Result<()> {
        use tokio::fs;
        
        let data_dir = self.base_dir.join("large");
        fs::create_dir_all(&data_dir).await?;
        
        // 创建大型文件
        let large_files = vec![
            ("1mb.txt", 1024 * 1024),
            ("10mb.txt", 10 * 1024 * 1024),
            ("100mb.txt", 100 * 1024 * 1024),
        ];
        
        for (filename, size) in large_files {
            let file_path = data_dir.join(filename);
            let content = "x".repeat(size);
            fs::write(&file_path, content).await?;
        }
        
        Ok(())
    }
    
    /// 创建二进制测试文件
    async fn create_binary_test_files(&self) -> Result<()> {
        use tokio::fs;
        
        let data_dir = self.base_dir.join("binary");
        fs::create_dir_all(&data_dir).await?;
        
        // 创建二进制文件
        let binary_files = vec![
            ("image.jpg", vec![0xFF, 0xD8, 0xFF, 0xE0]), // JPEG header
            ("archive.zip", vec![0x50, 0x4B, 0x03, 0x04]), // ZIP header
            ("executable", vec![0x7F, 0x45, 0x4C, 0x46]), // ELF header
            ("random.bin", (0..1024).map(|i| (i % 256) as u8).collect::<Vec<u8>>()),
        ];
        
        for (filename, data) in binary_files {
            let file_path = data_dir.join(filename);
            fs::write(&file_path, data).await?;
        }
        
        Ok(())
    }
    
    /// 创建特殊测试文件
    async fn create_special_test_files(&self) -> Result<()> {
        use tokio::fs;
        
        let data_dir = self.base_dir.join("special");
        fs::create_dir_all(&data_dir).await?;
        
        // 创建特殊文件
        let special_files = vec![
            ("newline_only.txt", "\n\n\n\n\n"),
            ("space_only.txt", "     "),
            ("zero_bytes.txt", vec![0u8; 1024]),
            ("max_path_name.txt", "x".repeat(255)), // Maximum filename length
            ("deep_path_file.txt", "Deep path file content"),
        ];
        
        for (filename, content) in special_files {
            let file_path = data_dir.join(filename);
            match content {
                serde_json::Value::String(s) => {
                    fs::write(&file_path, s).await?;
                }
                serde_json::Value::Array(bytes) => {
                    let byte_data: Vec<u8> = bytes.iter().map(|b| b.as_u64().unwrap() as u8).collect();
                    fs::write(&file_path, byte_data).await?;
                }
                _ => unreachable!(),
            }
        }
        
        // 创建深层路径文件
        let deep_dir = data_dir.join("deep").join("nested").join("path").join("structure");
        fs::create_dir_all(&deep_dir).await?;
        fs::write(deep_dir.join("deep_path_file.txt"), "Deep path file content").await?;
        
        Ok(())
    }
    
    /// 创建目录结构
    async fn create_directory_structures(&self) -> Result<()> {
        use tokio::fs;
        
        let structures_dir = self.base_dir.join("structures");
        fs::create_dir_all(&structures_dir).await?;
        
        // 创建扁平目录结构
        let flat_dir = structures_dir.join("flat");
        fs::create_dir_all(&flat_dir).await?;
        
        for i in 0..10 {
            let content = format!("Flat file {} content", i);
            fs::write(flat_dir.join(format!("file_{}.txt", i)), content).await?;
        }
        
        // 创建嵌套目录结构
        let nested_dir = structures_dir.join("nested");
        fs::create_dir_all(&nested_dir).await?;
        
        for level in 0..3 {
            for item in 0..3 {
                let dir_path = nested_dir.join(format!("level_{}", level)).join(format!("item_{}", item));
                fs::create_dir_all(&dir_path).await?;
                
                for file in 0..2 {
                    let content = format!("Nested file content - level {}, item {}, file {}", level, item, file);
                    fs::write(dir_path.join(format!("file_{}.txt", file)), content).await?;
                }
            }
        }
        
        // 创建混合目录结构
        let mixed_dir = structures_dir.join("mixed");
        fs::create_dir_all(&mixed_dir).await?;
        
        // 创建文件和目录混合的结构
        fs::write(mixed_dir.join("root_file.txt"), "Root file content").await?;
        
        let subdirs = vec!["documents", "images", "videos", "music"];
        for subdir in subdirs {
            let subdir_path = mixed_dir.join(subdir);
            fs::create_dir_all(&subdir_path).await?;
            
            // 在每个子目录中创建文件
            for i in 0..3 {
                let content = format!("{} file {} content", subdir, i);
                fs::write(subdir_path.join(format!("{}_{}.txt", subdir, i)), content).await?;
            }
        }
        
        Ok(())
    }
    
    /// 清理测试数据
    pub async fn cleanup(&self) -> Result<()> {
        use tokio::fs;
        
        if self.base_dir.exists() {
            fs::remove_dir_all(&self.base_dir).await?;
        }
        
        Ok(())
    }
    
    /// 复制测试数据到指定目录
    pub async fn copy_to<P: AsRef<std::path::Path>>(&self, target_dir: P) -> Result<()> {
        use tokio::fs;
        
        let target_dir = target_dir.as_ref();
        fs::create_dir_all(target_dir).await?;
        
        // 递归复制目录
        self.copy_dir_recursive(&self.base_dir, target_dir).await?;
        
        Ok(())
    }
    
    /// 递归复制目录
    async fn copy_dir_recursive(&self, src: &Path, dst: &Path) -> Result<()> {
        use tokio::fs;
        
        if !src.exists() {
            return Ok(());
        }
        
        if src.is_dir() {
            fs::create_dir_all(dst).await?;
            
            let mut entries = fs::read_dir(src).await?;
            while let Some(entry) = entries.next_entry().await? {
                let src_path = entry.path();
                let dst_path = dst.join(entry.file_name());
                
                if src_path.is_dir() {
                    self.copy_dir_recursive(&src_path, &dst_path).await?;
                } else {
                    fs::copy(&src_path, &dst_path).await?;
                }
            }
        } else {
            fs::copy(src, dst).await?;
        }
        
        Ok(())
    }
}

impl Default for TestDataManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 便捷函数：创建所有测试数据
pub async fn create_all_test_data() -> Result<()> {
    let manager = TestDataManager::new();
    manager.create_all_test_data().await
}

/// 便捷函数：清理所有测试数据
pub async fn cleanup_test_data() -> Result<()> {
    let manager = TestDataManager::new();
    manager.cleanup().await
}

/// 便捷函数：复制测试数据
pub async fn copy_test_data<P: AsRef<std::path::Path>>(target_dir: P) -> Result<()> {
    let manager = TestDataManager::new();
    manager.copy_to(target_dir).await
}