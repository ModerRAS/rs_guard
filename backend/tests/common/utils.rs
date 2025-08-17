//! 测试工具函数
//! 
//! 这个模块提供了各种测试工具函数，包括：
//! - 字符串工具
//! - 时间工具
//! - 随机工具
//! - 路径工具

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use rand::{thread_rng, Rng};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// 字符串工具
pub struct StringUtils;

impl StringUtils {
    /// 生成随机字符串
    pub fn random_string(length: usize) -> String {
        thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }
    
    /// 生成随机文件名
    pub fn random_filename() -> String {
        format!("{}_{}.txt", 
            Self::random_string(8),
            Utc::now().timestamp()
        )
    }
    
    /// 生成随机邮箱
    pub fn random_email() -> String {
        format!("{}@{}.com", 
            Self::random_string(10),
            Self::random_string(5)
        )
    }
    
    /// 截断字符串
    pub fn truncate(text: &str, max_length: usize) -> String {
        if text.len() <= max_length {
            text.to_string()
        } else {
            format!("{}...", &text[..max_length.saturating_sub(3)])
        }
    }
    
    /// 移除空白字符
    pub fn trim_whitespace(text: &str) -> String {
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    }
    
    /// 转换为蛇形命名
    pub fn to_snake_case(text: &str) -> String {
        text.chars()
            .map(|c| if c.is_uppercase() {
                format!("_{}", c.to_lowercase())
            } else {
                c.to_string()
            })
            .collect::<String>()
            .trim_start_matches('_')
            .to_string()
    }
    
    /// 转换为驼峰命名
    pub fn to_camel_case(text: &str) -> String {
        let parts: Vec<&str> = text.split('_').collect();
        parts.iter()
            .enumerate()
            .map(|(i, part)| {
                if i == 0 {
                    part.to_string()
                } else {
                    part.chars()
                        .enumerate()
                        .map(|(j, c)| if j == 0 { c.to_uppercase().to_string() } else { c.to_string() })
                        .collect()
                }
            })
            .collect()
    }
}

/// 时间工具
pub struct TimeUtils;

impl TimeUtils {
    /// 获取当前时间戳
    pub fn timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
    
    /// 获取当前时间戳（毫秒）
    pub fn timestamp_ms() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    }
    
    /// 格式化时间戳
    pub fn format_timestamp(timestamp: u64) -> String {
        let datetime = DateTime::from_timestamp(timestamp as i64, 0)
            .unwrap_or_default();
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }
    
    /// 获取 ISO 8601 格式时间
    pub fn iso8601_now() -> String {
        Utc::now().to_rfc3339()
    }
    
    /// 等待指定时间
    pub async fn sleep(duration: Duration) {
        tokio::time::sleep(duration).await;
    }
    
    /// 测量函数执行时间
    pub async fn measure<F, Fut, R>(func: F) -> (R, Duration)
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = R>,
    {
        let start = std::time::Instant::now();
        let result = func().await;
        let duration = start.elapsed();
        (result, duration)
    }
}

/// 随机工具
pub struct RandomUtils;

impl RandomUtils {
    /// 生成随机数
    pub fn random<T>() -> T
    where
        T: rand::distributions::uniform::SampleUniform,
        T: rand::distributions::uniform::SampleBorrow<T>,
        T: PartialOrd,
    {
        thread_rng().gen()
    }
    
    /// 生成指定范围内的随机数
    pub fn random_range<T>(min: T, max: T) -> T
    where
        T: rand::distributions::uniform::SampleUniform,
        T: rand::distributions::uniform::SampleBorrow<T>,
        T: PartialOrd,
    {
        thread_rng().gen_range(min..max)
    }
    
    /// 生成随机布尔值
    pub fn random_bool() -> bool {
        thread_rng().gen_bool(0.5)
    }
    
    /// 生成随机浮点数
    pub fn random_f64() -> f64 {
        thread_rng().gen()
    }
    
    /// 生成随机选择
    pub fn random_choice<T>(choices: &[T]) -> Option<&T> {
        if choices.is_empty() {
            None
        } else {
            Some(&choices[thread_rng().gen_range(0..choices.len())])
        }
    }
    
    /// 随机打乱向量
    pub fn shuffle<T>(vec: &mut [T]) {
        thread_rng().shuffle(vec);
    }
    
    /// 生成随机 UUID
    pub fn random_uuid() -> Uuid {
        Uuid::new_v4()
    }
}

/// 路径工具
pub struct PathUtils;

impl PathUtils {
    /// 创建测试路径
    pub fn test_path() -> PathBuf {
        std::env::temp_dir()
            .join("rs_guard_tests")
            .join(Uuid::new_v4().to_string())
    }
    
    /// 创建测试文件路径
    pub fn test_file_path(filename: &str) -> PathBuf {
        Self::test_path().join(filename)
    }
    
    /// 确保目录存在
    pub fn ensure_dir_exists<P: AsRef<Path>>(path: P) -> Result<()> {
        std::fs::create_dir_all(path)?;
        Ok(())
    }
    
    /// 删除目录及其内容
    pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> Result<()> {
        let path = path.as_ref();
        if path.exists() {
            std::fs::remove_dir_all(path)?;
        }
        Ok(())
    }
    
    /// 复制文件
    pub fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
        std::fs::copy(from, to)?;
        Ok(())
    }
    
    /// 移动文件
    pub fn move_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
        std::fs::rename(from, to)?;
        Ok(())
    }
    
    /// 获取文件大小
    pub fn file_size<P: AsRef<Path>>(path: P) -> Result<u64> {
        Ok(std::fs::metadata(path)?.len())
    }
    
    /// 检查文件是否存在
    pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().exists()
    }
    
    /// 检查是文件
    pub fn is_file<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().is_file()
    }
    
    /// 检查是目录
    pub fn is_dir<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().is_dir()
    }
    
    /// 读取文件内容
    pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
        Ok(std::fs::read_to_string(path)?)
    }
    
    /// 写入文件内容
    pub fn write_file<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, content: C) -> Result<()> {
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// 追加文件内容
    pub fn append_file<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, content: C) -> Result<()> {
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?
            .write_all(content.as_ref())?;
        Ok(())
    }
    
    /// 获取目录中的所有文件
    pub fn list_files<P: AsRef<Path>>(dir: P) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                files.push(path);
            }
        }
        Ok(files)
    }
    
    /// 获取目录中的所有子目录
    pub fn list_dirs<P: AsRef<Path>>(dir: P) -> Result<Vec<PathBuf>> {
        let mut dirs = Vec::new();
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path);
            }
        }
        Ok(dirs)
    }
}

/// 环境工具
pub struct EnvUtils;

impl EnvUtils {
    /// 获取环境变量
    pub fn get_var(key: &str) -> Option<String> {
        std::env::var(key).ok()
    }
    
    /// 设置环境变量
    pub fn set_var(key: &str, value: &str) {
        std::env::set_var(key, value);
    }
    
    /// 移除环境变量
    pub fn remove_var(key: &str) {
        std::env::remove_var(key);
    }
    
    /// 检查环境变量是否存在
    pub fn var_exists(key: &str) -> bool {
        std::env::var(key).is_ok()
    }
    
    /// 获取当前工作目录
    pub fn current_dir() -> Result<PathBuf> {
        Ok(std::env::current_dir()?)
    }
    
    /// 设置当前工作目录
    pub fn set_current_dir<P: AsRef<Path>>(path: P) -> Result<()> {
        std::env::set_current_dir(path)?;
        Ok(())
    }
    
    /// 获取程序路径
    pub fn current_exe() -> Result<PathBuf> {
        Ok(std::env::current_exe()?)
    }
}

/// 日志工具
pub struct LogUtils;

impl LogUtils {
    /// 记录测试开始
    pub fn test_start(test_name: &str) {
        println!("🧪 开始测试: {}", test_name);
    }
    
    /// 记录测试成功
    pub fn test_success(test_name: &str) {
        println!("✅ 测试成功: {}", test_name);
    }
    
    /// 记录测试失败
    pub fn test_failure(test_name: &str, error: &str) {
        println!("❌ 测试失败: {} - {}", test_name, error);
    }
    
    /// 记录测试跳过
    pub fn test_skip(test_name: &str, reason: &str) {
        println!("⏭️  跳过测试: {} - {}", test_name, reason);
    }
    
    /// 记录信息
    pub fn info(message: &str) {
        println!("ℹ️  {}", message);
    }
    
    /// 记录警告
    pub fn warn(message: &str) {
        println!("⚠️  {}", message);
    }
    
    /// 记录错误
    pub fn error(message: &str) {
        println!("🚨 {}", message);
    }
    
    /// 记录调试信息
    pub fn debug(message: &str) {
        if std::env::var("RUST_LOG").unwrap_or_default() == "debug" {
            println!("🔍 {}", message);
        }
    }
}

/// 测试辅助宏
#[macro_export]
macro_rules! measure_time {
    ($block:block) => {{
        let start = std::time::Instant::now();
        let result = $block;
        let duration = start.elapsed();
        (result, duration)
    }};
}

#[macro_export]
macro_rules! retry_until {
    ($condition:expr, $timeout_ms:expr, $interval_ms:expr) => {{
        let start = std::time::Instant::now();
        let timeout = std::time::Duration::from_millis($timeout_ms);
        let interval = std::time::Duration::from_millis($interval_ms);
        
        let mut result = $condition;
        while start.elapsed() < timeout && !result {
            std::thread::sleep(interval);
            result = $condition;
        }
        
        result
    }};
}

#[macro_export]
macro_rules! assert_eventually {
    ($condition:expr, $timeout_ms:expr, $interval_ms:expr) => {
        let result = $crate::retry_until!($condition, $timeout_ms, $interval_ms);
        assert!(result, "Condition not met within {}ms", $timeout_ms);
    };
}

#[macro_export]
macro_rules! with_temp_dir {
    ($block:block) => {{
        let temp_dir = $crate::PathUtils::test_path();
        $crate::PathUtils::ensure_dir_exists(&temp_dir).unwrap();
        
        let result = {
            let temp_dir = &temp_dir;
            $block
        };
        
        $crate::PathUtils::remove_dir_all(&temp_dir).unwrap();
        result
    }};
}