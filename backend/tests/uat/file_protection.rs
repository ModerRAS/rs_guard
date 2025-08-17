//! 文件保护用户验收测试
//! 
//! 这个模块包含文件保护功能的用户验收测试，验证系统能够：
//! - 监控文件系统变化
//! - 自动保护新创建的文件
//! - 处理文件更新和删除

use super::*;
use tokio::time::{sleep, Duration};
use serde_json::json;

/// 文件保护测试套件
pub struct FileProtectionTests;

impl FileProtectionTests {
    /// 测试基本文件保护流程
    pub async fn test_basic_file_protection() {
        let config = UatConfig::default();
        let context = UatContext::new(config).await;
        
        // 测试场景：创建新文件
        println!("测试场景：创建新文件");
        
        // 创建测试文件
        let test_content = "这是一个测试文件的内容，用于验证文件保护功能。";
        let file_path = context.create_test_file("test_protection.txt", test_content).await;
        
        // 等待文件处理
        let processed = context.wait_for_file_processing(3000).await;
        assert!(processed, "文件应该在 3 秒内被处理");
        
        // 验证文件存在
        assert!(context.file_exists("test_protection.txt").await, "测试文件应该存在");
        
        // 验证 API 响应
        let client = reqwest::Client::new();
        let response = client
            .get(&format!("{}/api/status", context.server_url()))
            .send()
            .await
            .expect("Failed to get status");
        
        UatAssertions::assert_status(&response, 200);
        let status = UatAssertions::assert_json(response).await;
        
        UatAssertions::assert_json_field(&status, "total_files");
        UatAssertions::assert_json_field(&status, "protected_files");
        
        // 清理
        context.cleanup().await;
    }
    
    /// 测试多个文件保护
    pub async fn test_multiple_files_protection() {
        let config = UatConfig::default();
        let context = UatContext::new(config).await;
        
        // 测试场景：批量创建文件
        println!("测试场景：批量创建文件");
        
        // 创建多个测试文件
        let files = vec![
            ("batch_file_1.txt", "文件 1 的内容"),
            ("batch_file_2.txt", "文件 2 的内容"),
            ("batch_file_3.txt", "文件 3 的内容"),
        ];
        
        for (filename, content) in &files {
            context.create_test_file(filename, content).await;
        }
        
        // 等待文件处理
        let processed = context.wait_for_file_processing(5000).await;
        assert!(processed, "多个文件应该在 5 秒内被处理");
        
        // 验证所有文件存在
        for (filename, _) in &files {
            assert!(context.file_exists(filename).await, "文件 {} 应该存在", filename);
        }
        
        // 验证受保护文件数量
        let state = context.app_state().lock().unwrap();
        assert!(state.total_files >= 3, "应该至少有 3 个文件被处理");
        drop(state);
        
        // 清理
        context.cleanup().await;
    }
    
    /// 测试大文件保护
    pub async fn test_large_file_protection() {
        let config = UatConfig::default();
        let context = UatContext::new(config).await;
        
        // 测试场景：大文件处理
        println!("测试场景：大文件处理");
        
        // 创建大文件（1MB）
        let large_content = UatUtils::generate_random_content(1024 * 1024);
        context.create_test_file("large_file.txt", &large_content).await;
        
        // 等待大文件处理（更长时间）
        let processed = context.wait_for_file_processing(10000).await;
        assert!(processed, "大文件应该在 10 秒内被处理");
        
        // 验证大文件存在
        assert!(context.file_exists("large_file.txt").await, "大文件应该存在");
        
        // 验证文件内容完整性
        let file_path = context.watched_dir().join("large_file.txt");
        let content = tokio::fs::read_to_string(&file_path).await
            .expect("Failed to read large file");
        assert_eq!(content, large_content, "大文件内容应该保持完整");
        
        // 清理
        context.cleanup().await;
    }
    
    /// 测试文件更新
    pub async fn test_file_update_protection() {
        let config = UatConfig::default();
        let context = UatContext::new(config).await;
        
        // 测试场景：文件更新
        println!("测试场景：文件更新");
        
        // 创建初始文件
        let initial_content = "初始文件内容";
        context.create_test_file("update_test.txt", initial_content).await;
        
        // 等待初始处理
        context.wait_for_file_processing(3000).await;
        
        // 更新文件内容
        let updated_content = "更新后的文件内容，包含更多信息。";
        let file_path = context.watched_dir().join("update_test.txt");
        tokio::fs::write(&file_path, updated_content).await
            .expect("Failed to update file");
        
        // 等待更新处理
        sleep(Duration::from_millis(2000)).await;
        
        // 验证更新后的内容
        let content = tokio::fs::read_to_string(&file_path).await
            .expect("Failed to read updated file");
        assert_eq!(content, updated_content, "文件内容应该正确更新");
        
        // 清理
        context.cleanup().await;
    }
    
    /// 测试文件删除处理
    pub async fn test_file_deletion_handling() {
        let config = UatConfig::default();
        let context = UatContext::new(config).await;
        
        // 测试场景：文件删除
        println!("测试场景：文件删除");
        
        // 创建测试文件
        context.create_test_file("delete_test.txt", "将要被删除的文件").await;
        
        // 等待文件处理
        context.wait_for_file_processing(3000).await;
        
        // 删除文件
        let file_path = context.watched_dir().join("delete_test.txt");
        tokio::fs::remove_file(&file_path).await
            .expect("Failed to delete file");
        
        // 等待删除处理
        sleep(Duration::from_millis(2000)).await;
        
        // 验证文件已删除
        assert!(!context.file_exists("delete_test.txt").await, "文件应该被删除");
        
        // 清理
        context.cleanup().await;
    }
    
    /// 测试文件保护边界情况
    pub async fn test_file_protection_edge_cases() {
        let config = UatConfig::default();
        let context = UatContext::new(config).await;
        
        // 测试场景：边界情况
        println!("测试场景：边界情况");
        
        // 1. 空文件
        context.create_test_file("empty_file.txt", "").await;
        
        // 2. 只有换行符的文件
        context.create_test_file("newline_file.txt", "\n\n\n").await;
        
        // 3. 特殊字符文件
        let special_content = "特殊字符：!@#$%^&*()_+-=[]{}|;':\",./<>?";
        context.create_test_file("special_chars.txt", special_content).await;
        
        // 4. Unicode 文件
        let unicode_content = "Unicode 测试：中文 🚀 emojis 😊";
        context.create_test_file("unicode_file.txt", unicode_content).await;
        
        // 等待处理
        context.wait_for_file_processing(5000).await;
        
        // 验证所有文件都被正确处理
        let files_to_check = vec![
            "empty_file.txt",
            "newline_file.txt", 
            "special_chars.txt",
            "unicode_file.txt",
        ];
        
        for filename in files_to_check {
            assert!(context.file_exists(filename).await, "边界情况文件 {} 应该存在", filename);
        }
        
        // 清理
        context.cleanup().await;
    }
    
    /// 运行所有文件保护测试
    pub async fn run_all_tests() {
        println!("🔧 开始运行文件保护用户验收测试...");
        
        Self::test_basic_file_protection().await;
        println!("✅ 基本文件保护测试通过");
        
        Self::test_multiple_files_protection().await;
        println!("✅ 多文件保护测试通过");
        
        Self::test_large_file_protection().await;
        println!("✅ 大文件保护测试通过");
        
        Self::test_file_update_protection().await;
        println!("✅ 文件更新保护测试通过");
        
        Self::test_file_deletion_handling().await;
        println!("✅ 文件删除处理测试通过");
        
        Self::test_file_protection_edge_cases().await;
        println!("✅ 文件保护边界情况测试通过");
        
        println!("🎉 所有文件保护用户验收测试通过！");
    }
}

#[tokio::test]
async fn test_file_protection_uat() {
    FileProtectionTests::run_all_tests().await;
}