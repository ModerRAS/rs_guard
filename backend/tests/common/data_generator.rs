//! 测试数据生成器
//! 
//! 这个模块提供了各种测试数据的生成功能，包括：
//! - 文件内容生成
//! - 文件名生成
//! - 目录结构生成
//! - 随机数据生成

use std::path::{Path, PathBuf};
use rand::{thread_rng, Rng};
use fake::{Fake, Faker};
use fake::faker::filesystem::en::FileName;
use fake::faker::lorem::en::{Sentence, Paragraph};
use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::Name;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::Result;

/// 测试数据生成器
pub struct TestDataGenerator {
    base_dir: PathBuf,
    rng: rand::rngs::ThreadRng,
}

impl TestDataGenerator {
    /// 创建新的测试数据生成器
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
            rng: thread_rng(),
        }
    }
    
    /// 生成随机文件名
    pub fn generate_filename(&mut self) -> String {
        let prefix: String = (0..8)
            .map(|_| self.rng.gen_range(b'a'..=b'z') as char)
            .collect();
        let extension = match self.rng.gen_range(0..5) {
            0 => "txt",
            1 => "json",
            2 => "csv",
            3 => "log",
            4 => "dat",
            _ => "txt",
        };
        format!("{}_{}.{}", prefix, Uuid::new_v4().to_string().split('-').next().unwrap(), extension)
    }
    
    /// 生成随机文本内容
    pub fn generate_text_content(&mut self, size: usize) -> String {
        if size == 0 {
            return String::new();
        }
        
        let mut content = String::new();
        let mut remaining = size;
        
        while remaining > 0 {
            let paragraph_size = std::cmp::min(remaining, self.rng.gen_range(50..200));
            let paragraph: String = Paragraph(paragraph_size).fake();
            content.push_str(&paragraph);
            content.push('\n');
            remaining = remaining.saturating_sub(paragraph.len() + 1);
        }
        
        content.truncate(size);
        content
    }
    
    /// 生成随机 JSON 内容
    pub fn generate_json_content(&mut self) -> String {
        let obj = serde_json::json!({
            "id": Uuid::new_v4(),
            "name": Name().fake::<String>(),
            "email": SafeEmail().fake::<String>(),
            "created_at": Utc::now().to_rfc3339(),
            "data": {
                "value": self.rng.gen_range(0..1000),
                "description": Sentence(5..10).fake::<String>(),
                "tags": (0..self.rng.gen_range(1..5))
                    .map(|_| Sentence(1..3).fake::<String>())
                    .collect::<Vec<_>>()
            }
        });
        
        serde_json::to_string_pretty(&obj).unwrap()
    }
    
    /// 生成随机 CSV 内容
    pub fn generate_csv_content(&mut self, rows: usize) -> String {
        let mut csv = String::new();
        
        // CSV 头部
        csv.push_str("id,name,email,age,active\n");
        
        // CSV 数据行
        for _ in 0..rows {
            let id = Uuid::new_v4();
            let name = Name().fake::<String>();
            let email = SafeEmail().fake::<String>();
            let age = self.rng.gen_range(18..80);
            let active = self.rng.gen_bool(0.8);
            
            csv.push_str(&format!("{},{},{},{},{}\n", id, name, email, age, active));
        }
        
        csv
    }
    
    /// 生成随机二进制数据
    pub fn generate_binary_data(&mut self, size: usize) -> Vec<u8> {
        (0..size).map(|_| self.rng.gen()).collect()
    }
    
    /// 创建测试文件
    pub async fn create_test_file(&mut self, filename: Option<String>, content: String) -> Result<PathBuf> {
        let filename = filename.unwrap_or_else(|| self.generate_filename());
        let file_path = self.base_dir.join(filename);
        
        // 确保目录存在
        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        // 写入文件
        tokio::fs::write(&file_path, content).await?;
        
        Ok(file_path)
    }
    
    /// 创建文本测试文件
    pub async fn create_text_file(&mut self, size: usize) -> Result<PathBuf> {
        let content = self.generate_text_content(size);
        self.create_test_file(None, content).await
    }
    
    /// 创建 JSON 测试文件
    pub async fn create_json_file(&mut self) -> Result<PathBuf> {
        let content = self.generate_json_content();
        self.create_test_file(None, content).await
    }
    
    /// 创建 CSV 测试文件
    pub async fn create_csv_file(&mut self, rows: usize) -> Result<PathBuf> {
        let content = self.generate_csv_content(rows);
        self.create_test_file(None, content).await
    }
    
    /// 创建二进制测试文件
    pub async fn create_binary_file(&mut self, size: usize) -> Result<PathBuf> {
        let content = self.generate_binary_data(size);
        let filename = self.generate_filename();
        let file_path = self.base_dir.join(filename);
        
        // 确保目录存在
        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        // 写入文件
        tokio::fs::write(&file_path, content).await?;
        
        Ok(file_path)
    }
    
    /// 创建空文件
    pub async fn create_empty_file(&mut self) -> Result<PathBuf> {
        self.create_test_file(None, String::new()).await
    }
    
    /// 创建目录结构
    pub async fn create_directory_structure(&mut self, structure: DirectoryStructure) -> Result<()> {
        match structure {
            DirectoryStructure::Flat { files } => {
                for _ in 0..files {
                    self.create_text_file(100).await?;
                }
            }
            DirectoryStructure::Nested { depth, breadth, files_per_dir } => {
                self.create_nested_directory(0, depth, breadth, files_per_dir, &self.base_dir).await?;
            }
            DirectoryStructure::Custom { paths } => {
                for path in paths {
                    let full_path = self.base_dir.join(path);
                    if full_path.extension().is_none() {
                        // 这是一个目录
                        tokio::fs::create_dir_all(&full_path).await?;
                    } else {
                        // 这是一个文件
                        if let Some(parent) = full_path.parent() {
                            tokio::fs::create_dir_all(parent).await?;
                        }
                        let content = self.generate_text_content(50);
                        tokio::fs::write(&full_path, content).await?;
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// 递归创建嵌套目录结构
    async fn create_nested_directory(
        &mut self,
        current_depth: usize,
        max_depth: usize,
        breadth: usize,
        files_per_dir: usize,
        base_dir: &Path,
    ) -> Result<()> {
        if current_depth >= max_depth {
            return Ok(());
        }
        
        // 在当前目录创建文件
        for _ in 0..files_per_dir {
            self.create_text_file(50).await?;
        }
        
        // 创建子目录
        for i in 0..breadth {
            let subdir_name = format!("subdir_{}_{}", current_depth, i);
            let subdir_path = base_dir.join(subdir_name);
            tokio::fs::create_dir_all(&subdir_path).await?;
            
            // 递归创建子目录的结构
            let mut sub_generator = TestDataGenerator::new(&subdir_path);
            sub_generator.create_nested_directory(
                current_depth + 1,
                max_depth,
                breadth,
                files_per_dir,
                &subdir_path,
            ).await?;
        }
        
        Ok(())
    }
    
    /// 创建测试文件集合
    pub async fn create_test_file_collection(&mut self, collection: FileCollection) -> Result<Vec<PathBuf>> {
        let mut created_files = Vec::new();
        
        match collection {
            FileCollection::Random { count, min_size, max_size } => {
                for _ in 0..count {
                    let size = self.rng.gen_range(min_size..max_size);
                    let file_path = self.create_text_file(size).await?;
                    created_files.push(file_path);
                }
            }
            FileCollection::VariedTypes { count_per_type } => {
                // 文本文件
                for _ in 0..count_per_type {
                    let file_path = self.create_text_file(100).await?;
                    created_files.push(file_path);
                }
                
                // JSON 文件
                for _ in 0..count_per_type {
                    let file_path = self.create_json_file().await?;
                    created_files.push(file_path);
                }
                
                // CSV 文件
                for _ in 0..count_per_type {
                    let file_path = self.create_csv_file(10).await?;
                    created_files.push(file_path);
                }
                
                // 二进制文件
                for _ in 0..count_per_type {
                    let file_path = self.create_binary_file(1024).await?;
                    created_files.push(file_path);
                }
            }
            FileCollection::Specific { files } => {
                for file_spec in files {
                    let file_path = match file_spec {
                        FileSpec::Text { size, name } => {
                            let content = self.generate_text_content(size);
                            self.create_test_file(name, content).await?
                        }
                        FileSpec::Binary { size, name } => {
                            let content = self.generate_binary_data(size);
                            let filename = name.unwrap_or_else(|| self.generate_filename());
                            let file_path = self.base_dir.join(filename);
                            
                            if let Some(parent) = file_path.parent() {
                                tokio::fs::create_dir_all(parent).await?;
                            }
                            
                            tokio::fs::write(&file_path, content).await?;
                            file_path
                        }
                    };
                    created_files.push(file_path);
                }
            }
        }
        
        Ok(created_files)
    }
    
    /// 获取生成的文件列表
    pub async fn list_generated_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        
        let mut entries = tokio::fs::read_dir(&self.base_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() {
                files.push(path);
            }
        }
        
        Ok(files)
    }
    
    /// 清理所有生成的文件
    pub async fn cleanup(&self) -> Result<()> {
        if self.base_dir.exists() {
            tokio::fs::remove_dir_all(&self.base_dir).await?;
        }
        Ok(())
    }
}

/// 目录结构类型
pub enum DirectoryStructure {
    /// 平坦目录结构
    Flat { files: usize },
    /// 嵌套目录结构
    Nested { depth: usize, breadth: usize, files_per_dir: usize },
    /// 自定义目录结构
    Custom { paths: Vec<String> },
}

/// 文件集合类型
pub enum FileCollection {
    /// 随机文件集合
    Random { count: usize, min_size: usize, max_size: usize },
    /// 多种类型文件集合
    VariedTypes { count_per_type: usize },
    /// 特定文件集合
    Specific { files: Vec<FileSpec> },
}

/// 文件规格
pub enum FileSpec {
    /// 文本文件
    Text { size: usize, name: Option<String> },
    /// 二进制文件
    Binary { size: usize, name: Option<String> },
}

/// 便捷函数：快速创建测试数据
pub async fn create_test_data(base_dir: &Path, file_count: usize) -> Result<Vec<PathBuf>> {
    let mut generator = TestDataGenerator::new(base_dir);
    
    let mut files = Vec::new();
    for _ in 0..file_count {
        let file_path = generator.create_text_file(100).await?;
        files.push(file_path);
    }
    
    Ok(files)
}

/// 便捷函数：创建测试目录结构
pub async fn create_test_directory_structure(base_dir: &Path) -> Result<()> {
    let mut generator = TestDataGenerator::new(base_dir);
    
    // 创建一个复杂的目录结构
    let structure = DirectoryStructure::Nested {
        depth: 3,
        breadth: 2,
        files_per_dir: 2,
    };
    
    generator.create_directory_structure(structure).await
}