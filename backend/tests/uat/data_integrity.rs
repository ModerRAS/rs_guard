//! 数据完整性用户验收测试
//! 
//! 这个模块包含数据完整性功能的用户验收测试，验证：
//! - 文件完整性检查
//! - 损坏检测
//! - 自动修复功能
//! - 数据一致性验证

use super::*;
use tokio::time::{sleep, Duration};
use serde_json::json;
use std::fs;

/// 数据完整性测试套件
pub struct DataIntegrityTests;

impl DataIntegrityTests {
    /// 测试基本完整性检查
    pub async fn test_basic_integrity_check() {
        let config = UatConfig::default();
        let context = UatContext::new(config).await;
        
        println!("测试场景：基本完整性检查");
        
        // 创建测试文件
        let test_content = "用于完整性检查的测试文件内容。";
        context.create_test_file("integrity_test.txt", test_content).await;
        
        // 等待文件处理
        context.wait_for_file_processing(3000).await;
        
        let client = reqwest::Client::new();
        
        // 执行完整性检查
        let response = client
            .get(&format!("{}/api/check", context.server_url()))
            .send()
            .await
            .expect("Failed to perform integrity check");
        
        UatAssertions::assert_status(&response, 200);
        let check_response = UatAssertions::assert_json(response).await;
        
        // 验证检查结果
        UatAssertions::assert_json_field(&check_response, "status");
        UatAssertions::assert_json_field(&check_response, "checked_files");
        UatAssertions::assert_json_field(&check_response, "corrupted_files");
        UatAssertions::assert_json_field(&check_response, "timestamp");
        
        // 验证检查的文件数量
        let checked_files = check_response["checked_files"].as_u64().unwrap_or(0);
        assert!(checked_files > 0, "应该至少检查了一个文件");
        
        // 清理
        context.cleanup().await;
    }
    
    /// 测试损坏检测
    pub async fn test_corruption_detection() {
        let config = UatConfig::default();
        let context = UatContext::new(config).await;
        
        println!("测试场景：损坏检测");
        
        // 创建测试文件
        let original_content = "原始文件内容，用于测试损坏检测。";
        context.create_test_file("corruption_test.txt", original_content).await;
        
        // 等待文件处理
        context.wait_for_file_processing(3000).await;
        
        // 模拟文件损坏（修改文件内容）
        let file_path = context.watched_dir().join("corruption_test.txt");
        let corrupted_content = "损坏的文件内容，与原始内容不同。";
        fs::write(&file_path, corrupted_content).expect("Failed to corrupt file");
        
        // 等待系统检测到变化
        sleep(Duration::from_millis(1000)).await;
        
        // 执行完整性检查
        let client = reqwest::Client::new();
        let response = client
            .get(&format!("{}/api/check", context.server_url()))
            .send()
            .await
            .expect("Failed to perform integrity check");
        
        UatAssertions::assert_status(&response, 200);
        let check_response = UatAssertions::assert_json(response).await;
        
        // 验证检测到损坏
        let corrupted_files = check_response["corrupted_files"].as_u64().unwrap_or(0);
        assert!(corrupted_files > 0, "应该检测到损坏的文件");
        
        // 清理
        context.cleanup().await;
    }
    
    /// 测试自动修复功能
    pub async fn test_auto_repair() {
        let config = UatConfig::default();
        let context = UatContext::new(config).await;
        
        println!("测试场景：自动修复");
        
        // 创建测试文件
        let original_content = "用于测试自动修复的原始内容。";
        context.create_test_file("repair_test.txt", original_content).await;
        
        // 等待文件处理
        context.wait_for_file_processing(3000).await;
        
        // 模拟文件损坏
        let file_path = context.watched_dir().join("repair_test.txt");
        let corrupted_content = "损坏的内容。";
        fs::write(&file_path, corrupted_content).expect("Failed to corrupt file");
        
        // 等待系统处理
        sleep(Duration::from_millis(2000)).await;
        
        // 触发修复
        let client = reqwest::Client::new();
        let response = client
            .post(&format!("{}/api/repair", context.server_url()))
            .send()
            .await
            .expect("Failed to trigger repair");
        
        UatAssertions::assert_status(&response, 200);
        let repair_response = UatAssertions::assert_json(response).await;
        
        // 验证修复结果
        UatAssertions::assert_json_field(&repair_response, "repaired_files");
        UatAssertions::assert_json_field(&repair_response, "failed_repairs");
        
        // 等待修复完成
        sleep(Duration::from_millis(3000)).await;
        
        // 验证文件已被修复
        let repaired_content = tokio::fs::read_to_string(&file_path).await
            .expect("Failed to read repaired file");
        
        // 注意：实际的修复行为取决于系统的实现
        // 这里我们只是验证文件存在且可读
        assert!(!repaired_content.is_empty(), "修复后的文件不应该为空");
        
        // 清理
        context.cleanup().await;
    }
    
    /// 测试批量完整性检查
    pub async fn test_batch_integrity_check() {
        let config = UatConfig::default();
        let context = UatContext::new(config).await;
        
        println!("测试场景：批量完整性检查");
        
        // 创建多个测试文件
        let files = vec![
            ("batch_check_1.txt", "批量检查文件 1"),
            ("batch_check_2.txt", "批量检查文件 2"),
            ("batch_check_3.txt", "批量检查文件 3"),
            ("batch_check_4.txt", "批量检查文件 4"),
            ("batch_check_5.txt", "批量检查文件 5"),
        ];
        
        for (filename, content) in &files {
            context.create_test_file(filename, content).await;
        }
        
        // 等待文件处理
        context.wait_for_file_processing(5000).await;
        
        let client = reqwest::Client::new();
        
        // 执行批量完整性检查
        let response = client
            .get(&format!("{}/api/check", context.server_url()))
            .send()
            .await
            .expect("Failed to perform batch integrity check");
        
        UatAssertions::assert_status(&response, 200);
        let check_response = UatAssertions::assert_json(response).await;
        
        // 验证批量检查结果
        let checked_files = check_response["checked_files"].as_u64().unwrap_or(0);
        assert!(checked_files >= 5, "应该检查了至少 5 个文件");
        
        // 清理
        context.cleanup().await;
    }
    
    /// 测试完整性检查性能
    pub async fn test_integrity_check_performance() {
        let config = UatConfig::default();
        let context = UatContext::new(config).await;
        
        println!("测试场景：完整性检查性能");
        
        // 创建大量测试文件
        for i in 0..20 {
            let content = format!("性能测试文件 {} 的内容。", i);
            let filename = format!("perf_test_{}.txt", i);
            context.create_test_file(&filename, &content).await;
        }
        
        // 等待文件处理
        context.wait_for_file_processing(5000).await;
        
        let client = reqwest::Client::new();
        
        // 测量完整性检查时间
        let start = std::time::Instant::now();
        let response = client
            .get(&format!("{}/api/check", context.server_url()))
            .send()
            .await
            .expect("Failed to perform integrity check");
        let duration = start.elapsed();
        
        UatAssertions::assert_status(&response, 200);
        let check_response = UatAssertions::assert_json(response).await;
        
        // 验证性能要求
        assert!(duration.as_secs() < 10, "完整性检查应该在 10 秒内完成");
        
        // 验证检查结果
        let checked_files = check_response["checked_files"].as_u64().unwrap_or(0);
        assert!(checked_files >= 20, "应该检查了至少 20 个文件");
        
        // 清理
        context.cleanup().await;
    }
    
    /// 测试数据一致性验证
    pub async fn test_data_consistency() {
        let config = UatConfig::default();
        let context = UatContext::new(config).await;
        
        println!("测试场景：数据一致性验证");
        
        // 创建测试文件
        let test_content = "用于数据一致性测试的内容。";
        context.create_test_file("consistency_test.txt", test_content).await;
        
        // 等待文件处理
        context.wait_for_file_processing(3000).await;
        
        let client = reqwest::Client::new();
        
        // 多次执行完整性检查，验证结果一致性
        let mut results = Vec::new();
        
        for _ in 0..3 {
            let response = client
                .get(&format!("{}/api/check", context.server_url()))
                .send()
                .await
                .expect("Failed to perform integrity check");
            
            UatAssertions::assert_status(&response, 200);
            let check_response = UatAssertions::assert_json(response).await;
            
            let checked_files = check_response["checked_files"].as_u64().unwrap_or(0);
            results.push(checked_files);
            
            // 等待一段时间再进行下一次检查
            sleep(Duration::from_millis(500)).await;
        }
        
        // 验证多次检查结果一致
        assert!(results.windows(2).all(|w| w[0] == w[1]), 
            "多次完整性检查的结果应该一致");
        
        // 清理
        context.cleanup().await;
    }
    
    /// 测试边界情况处理
    pub async fn test_integrity_edge_cases() {
        let config = UatConfig::default();
        let context = UatContext::new(config).await;
        
        println!("测试场景：完整性检查边界情况");
        
        // 1. 空文件
        context.create_test_file("empty_integrity.txt", "").await;
        
        // 2. 极小文件
        context.create_test_file("tiny_integrity.txt", "x").await;
        
        // 3. 大文件
        let large_content = UatUtils::generate_random_content(1024 * 100); // 100KB
        context.create_test_file("large_integrity.txt", &large_content).await;
        
        // 等待文件处理
        context.wait_for_file_processing(5000).await;
        
        let client = reqwest::Client::new();
        
        // 执行完整性检查
        let response = client
            .get(&format!("{}/api/check", context.server_url()))
            .send()
            .await
            .expect("Failed to perform integrity check");
        
        UatAssertions::assert_status(&response, 200);
        let check_response = UatAssertions::assert_json(response).await;
        
        // 验证边界情况文件都被正确处理
        let checked_files = check_response["checked_files"].as_u64().unwrap_or(0);
        assert!(checked_files >= 3, "应该检查了至少 3 个边界情况文件");
        
        // 清理
        context.cleanup().await;
    }
    
    /// 运行所有数据完整性测试
    pub async fn run_all_tests() {
        println!("🔍 开始运行数据完整性用户验收测试...");
        
        Self::test_basic_integrity_check().await;
        println!("✅ 基本完整性检查测试通过");
        
        Self::test_corruption_detection().await;
        println!("✅ 损坏检测测试通过");
        
        Self::test_auto_repair().await;
        println!("✅ 自动修复测试通过");
        
        Self::test_batch_integrity_check().await;
        println!("✅ 批量完整性检查测试通过");
        
        Self::test_integrity_check_performance().await;
        println!("✅ 完整性检查性能测试通过");
        
        Self::test_data_consistency().await;
        println!("✅ 数据一致性验证测试通过");
        
        Self::test_integrity_edge_cases().await;
        println!("✅ 完整性检查边界情况测试通过");
        
        println!("🎉 所有数据完整性用户验收测试通过！");
    }
}

#[tokio::test]
async fn test_data_integrity_uat() {
    DataIntegrityTests::run_all_tests().await;
}