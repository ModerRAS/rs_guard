//! 测试断言工具
//! 
//! 这个模块提供了各种测试断言工具，包括：
//! - 文件系统断言
//! - JSON 断言
//! - HTTP 断言
//! - 时间断言
//! - 自定义断言

use std::path::{Path, PathBuf};
use std::time::Duration;
use serde_json::Value;
use anyhow::Result;

/// 文件系统断言
pub struct FileAssertions;

impl FileAssertions {
    /// 断言文件存在
    pub fn assert_exists<P: AsRef<Path>>(path: P) {
        let path = path.as_ref();
        assert!(path.exists(), "Path '{}' should exist", path.display());
    }
    
    /// 断言文件不存在
    pub fn assert_not_exists<P: AsRef<Path>>(path: P) {
        let path = path.as_ref();
        assert!(!path.exists(), "Path '{}' should not exist", path.display());
    }
    
    /// 断言文件是目录
    pub fn assert_is_dir<P: AsRef<Path>>(path: P) {
        let path = path.as_ref();
        assert!(path.is_dir(), "Path '{}' should be a directory", path.display());
    }
    
    /// 断言文件是文件
    pub fn assert_is_file<P: AsRef<Path>>(path: P) {
        let path = path.as_ref();
        assert!(path.is_file(), "Path '{}' should be a file", path.display());
    }
    
    /// 断言文件大小
    pub fn assert_file_size<P: AsRef<Path>>(path: P, expected_size: u64) {
        let path = path.as_ref();
        let metadata = path.metadata().expect("Failed to get file metadata");
        assert_eq!(metadata.len(), expected_size, "File '{}' should have size {} bytes", path.display(), expected_size);
    }
    
    /// 断言文件内容
    pub fn assert_file_content<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, expected_content: C) {
        let path = path.as_ref();
        let content = std::fs::read(path).expect("Failed to read file");
        assert_eq!(content, expected_content.as_ref(), "File '{}' content mismatch", path.display());
    }
    
    /// 断言文件包含内容
    pub fn assert_file_contains<P: AsRef<Path>, S: AsRef<str>>(path: P, expected_content: S) {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path).expect("Failed to read file");
        assert!(content.contains(expected_content.as_ref()), "File '{}' should contain '{}'", path.display(), expected_content.as_ref());
    }
    
    /// 断言目录包含特定文件
    pub fn assert_dir_contains<P: AsRef<Path>, F: AsRef<str>>(path: P, filename: F) {
        let path = path.as_ref();
        let filename = filename.as_ref();
        let entries = std::fs::read_dir(path).expect("Failed to read directory");
        let found = entries.any(|entry| {
            if let Ok(entry) = entry {
                entry.file_name().to_string_lossy() == filename
            } else {
                false
            }
        });
        assert!(found, "Directory '{}' should contain file '{}'", path.display(), filename);
    }
    
    /// 断言文件权限
    pub fn assert_file_permissions<P: AsRef<Path>>(path: P, expected_permissions: u32) {
        let path = path.as_ref();
        let metadata = path.metadata().expect("Failed to get file metadata");
        let permissions = metadata.permissions().mode() & 0o777;
        assert_eq!(permissions, expected_permissions, "File '{}' should have permissions {:o}", path.display(), expected_permissions);
    }
    
    /// 断言文件修改时间在指定范围内
    pub fn assert_modified_within<P: AsRef<Path>>(path: P, duration: Duration) {
        let path = path.as_ref();
        let metadata = path.metadata().expect("Failed to get file metadata");
        let modified_time = metadata.modified().expect("Failed to get modified time");
        let now = std::time::SystemTime::now();
        let elapsed = now.duration_since(modified_time).unwrap_or_else(|_| modified_time.duration_since(now).unwrap());
        assert!(elapsed <= duration, "File '{}' was modified too long ago", path.display());
    }
}

/// JSON 断言
pub struct JsonAssertions;

impl JsonAssertions {
    /// 断言 JSON 值相等
    pub fn assert_eq_json(actual: &Value, expected: &Value) {
        assert_eq!(actual, expected, "JSON values should be equal");
    }
    
    /// 断言 JSON 包含特定字段
    pub fn assert_has_field(json: &Value, field: &str) {
        assert!(json.get(field).is_some(), "JSON should have field '{}'", field);
    }
    
    /// 断言 JSON 字段值相等
    pub fn assert_field_eq(json: &Value, field: &str, expected: &Value) {
        let actual = json.get(field).unwrap_or_else(|| panic!("JSON should have field '{}'", field));
        assert_eq!(actual, expected, "JSON field '{}' should be equal to expected value", field);
    }
    
    /// 断言 JSON 字段类型
    pub fn assert_field_type(json: &Value, field: &str, expected_type: &str) {
        let value = json.get(field).unwrap_or_else(|| panic!("JSON should have field '{}'", field));
        let actual_type = match value {
            Value::Null => "null",
            Value::Bool(_) => "boolean",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
        };
        assert_eq!(actual_type, expected_type, "JSON field '{}' should be of type '{}'", field, expected_type);
    }
    
    /// 断言 JSON 数组长度
    pub fn assert_array_len(json: &Value, expected_len: usize) {
        if let Value::Array(array) = json {
            assert_eq!(array.len(), expected_len, "JSON array should have length {}", expected_len);
        } else {
            panic!("JSON value should be an array");
        }
    }
    
    /// 断言 JSON 对象包含所有必需字段
    pub fn assert_has_fields(json: &Value, fields: &[&str]) {
        for field in fields {
            assert!(json.get(field).is_some(), "JSON should have field '{}'", field);
        }
    }
    
    /// 断言 JSON 匹配模式
    pub fn assert_matches_pattern(json: &Value, pattern: &str) {
        let json_str = serde_json::to_string(json).unwrap();
        assert!(json_str.contains(pattern), "JSON should match pattern '{}'", pattern);
    }
}

/// HTTP 断言
pub struct HttpAssertions;

impl HttpAssertions {
    /// 断言 HTTP 状态码
    pub fn assert_status_code(status: u16, expected: u16) {
        assert_eq!(status, expected, "HTTP status code should be {}", expected);
    }
    
    /// 断言 HTTP 响应头
    pub fn assert_has_header(headers: &reqwest::header::HeaderMap, header_name: &str) {
        assert!(headers.contains_key(header_name), "HTTP response should have header '{}'", header_name);
    }
    
    /// 断言 HTTP 响应头值
    pub fn assert_header_value(headers: &reqwest::header::HeaderMap, header_name: &str, expected_value: &str) {
        let actual_value = headers.get(header_name)
            .unwrap_or_else(|| panic!("HTTP response should have header '{}'", header_name))
            .to_str()
            .unwrap_or_else(|_| panic!("Header '{}' should be valid UTF-8", header_name));
        assert_eq!(actual_value, expected_value, "Header '{}' should have value '{}'", header_name, expected_value);
    }
    
    /// 断言 HTTP 响应体长度
    pub fn assert_response_body_length(body: &str, expected_length: usize) {
        assert_eq!(body.len(), expected_length, "HTTP response body should have length {}", expected_length);
    }
    
    /// 断言 HTTP 响应体包含
    pub fn assert_response_body_contains(body: &str, expected_content: &str) {
        assert!(body.contains(expected_content), "HTTP response body should contain '{}'", expected_content);
    }
    
    /// 断言 HTTP 响应时间是 JSON
    pub fn assert_response_is_json(headers: &reqwest::header::HeaderMap) {
        let content_type = headers.get("content-type")
            .unwrap_or_else(|| panic!("HTTP response should have content-type header"))
            .to_str()
            .unwrap_or_else(|_| panic!("Content-Type header should be valid UTF-8"));
        assert!(content_type.contains("application/json"), "HTTP response should be JSON");
    }
    
    /// 断言 HTTP 响应时间是成功的
    pub fn assert_success_status(status: u16) {
        assert!(status >= 200 && status < 300, "HTTP status {} should be successful", status);
    }
}

/// 时间断言
pub struct TimeAssertions;

impl TimeAssertions {
    /// 断言时间在指定范围内
    pub fn assert_within_range(actual: Duration, expected: Duration, tolerance: Duration) {
        let diff = if actual > expected {
            actual - expected
        } else {
            expected - actual
        };
        assert!(diff <= tolerance, "Duration {:?} should be within {:?} of {:?}", actual, tolerance, expected);
    }
    
    /// 断言时间大于指定值
    pub fn assert_greater_than(actual: Duration, minimum: Duration) {
        assert!(actual > minimum, "Duration {:?} should be greater than {:?}", actual, minimum);
    }
    
    /// 断言时间小于指定值
    pub fn assert_less_than(actual: Duration, maximum: Duration) {
        assert!(actual < maximum, "Duration {:?} should be less than {:?}", actual, maximum);
    }
}

/// 性能断言
pub struct PerformanceAssertions;

impl PerformanceAssertions {
    /// 断言响应时间在可接受范围内
    pub fn assert_response_time_acceptable(response_time: Duration, max_acceptable: Duration) {
        assert!(response_time <= max_acceptable, "Response time {:?} should be within acceptable limit {:?}", response_time, max_acceptable);
    }
    
    /// 断言内存使用在可接受范围内
    pub fn assert_memory_usage_acceptable(memory_usage: usize, max_acceptable: usize) {
        assert!(memory_usage <= max_acceptable, "Memory usage {} bytes should be within acceptable limit {} bytes", memory_usage, max_acceptable);
    }
    
    /// 断言并发性能
    pub fn assert_concurrent_performance(success_count: usize, total_count: usize, min_success_rate: f64) {
        let success_rate = (success_count as f64) / (total_count as f64);
        assert!(success_rate >= min_success_rate, "Success rate {} should be at least {}", success_rate, min_success_rate);
    }
}

/// 自定义断言
pub struct CustomAssertions;

impl CustomAssertions {
    /// 断言两个浮点数在误差范围内相等
    pub fn assert_almost_equal(actual: f64, expected: f64, tolerance: f64) {
        let diff = (actual - expected).abs();
        assert!(diff <= tolerance, "Value {} should be almost equal to {} with tolerance {}", actual, expected, tolerance);
    }
    
    /// 断言百分比在误差范围内
    pub fn assert_percentage_within_range(actual: f64, expected: f64, tolerance_percent: f64) {
        let tolerance = expected * (tolerance_percent / 100.0);
        Self::assert_almost_equal(actual, expected, tolerance);
    }
    
    /// 断言字符串匹配正则表达式
    pub fn assert_matches_regex(text: &str, pattern: &str) {
        let regex = regex::Regex::new(pattern).unwrap_or_else(|_| panic!("Invalid regex pattern: {}", pattern));
        assert!(regex.is_match(text), "Text '{}' should match regex pattern '{}'", text, pattern);
    }
    
    /// 断言集合包含所有元素
    pub fn assert_contains_all<T: PartialEq>(collection: &[T], elements: &[T]) {
        for element in elements {
            assert!(collection.contains(element), "Collection should contain element {:?}", element);
        }
    }
    
    /// 断言集合不包含任何元素
    pub fn assert_contains_none<T: PartialEq>(collection: &[T], elements: &[T]) {
        for element in elements {
            assert!(!collection.contains(element), "Collection should not contain element {:?}", element);
        }
    }
}

/// 便捷函数：创建测试断言宏
#[macro_export]
macro_rules! assert_file_exists {
    ($path:expr) => {
        $crate::common::assertions::FileAssertions::assert_exists($path);
    };
}

#[macro_export]
macro_rules! assert_json_eq {
    ($actual:expr, $expected:expr) => {
        $crate::common::assertions::JsonAssertions::assert_eq_json($actual, $expected);
    };
}

#[macro_export]
macro_rules! assert_status_code {
    ($status:expr, $expected:expr) => {
        $crate::common::assertions::HttpAssertions::assert_status_code($status, $expected);
    };
}

#[macro_export]
macro_rules! assert_within_range {
    ($actual:expr, $expected:expr, $tolerance:expr) => {
        $crate::common::assertions::TimeAssertions::assert_within_range($actual, $expected, $tolerance);
    };
}

/// 测试断言结果
#[derive(Debug)]
pub struct AssertionResult {
    pub passed: bool,
    pub message: String,
    pub details: Option<String>,
}

impl AssertionResult {
    pub fn new(passed: bool, message: &str) -> Self {
        Self {
            passed,
            message: message.to_string(),
            details: None,
        }
    }
    
    pub fn with_details(mut self, details: &str) -> Self {
        self.details = Some(details.to_string());
        self
    }
    
    pub fn success(message: &str) -> Self {
        Self::new(true, message)
    }
    
    pub fn failure(message: &str) -> Self {
        Self::new(false, message)
    }
}

/// 批量断言执行器
pub struct AssertionBatch {
    results: Vec<AssertionResult>,
}

impl AssertionBatch {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }
    
    pub fn add_assertion<F>(&mut self, assertion: F, description: &str)
    where
        F: FnOnce() -> AssertionResult,
    {
        let result = assertion();
        self.results.push(AssertionResult::new(result.passed, description));
    }
    
    pub fn execute_all(self) -> Vec<AssertionResult> {
        self.results
    }
    
    pub fn assert_all_passed(&self) {
        let failures: Vec<_> = self.results.iter()
            .filter(|r| !r.passed)
            .collect();
        
        if !failures.is_empty() {
            let failure_messages: Vec<String> = failures.iter()
                .map(|f| format!("✗ {}", f.message))
                .collect();
            panic!("Assertion failures:\n{}", failure_messages.join("\n"));
        }
    }
}

impl Default for AssertionBatch {
    fn default() -> Self {
        Self::new()
    }
}