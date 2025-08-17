//! 测试报告生成器
//! 
//! 这个模块提供了测试报告的生成功能，支持多种格式：
//! - HTML 报告
//! - JSON 报告
//! - JUnit XML 报告
//! - 控制台输出

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde_json::Value;
use chrono::{DateTime, Utc};
use anyhow::Result;

/// 报告格式枚举
#[derive(Debug, Clone)]
pub enum ReportFormat {
    Html,
    Json,
    JUnit,
    Console,
}

/// 测试报告生成器
pub struct TestReportGenerator {
    project_name: String,
    test_results: Vec<TestResult>,
    metadata: HashMap<String, Value>,
    generated_at: DateTime<Utc>,
}

impl TestReportGenerator {
    /// 创建新的报告生成器
    pub fn new(project_name: String) -> Self {
        Self {
            project_name,
            test_results: Vec::new(),
            metadata: HashMap::new(),
            generated_at: Utc::now(),
        }
    }
    
    /// 添加测试结果
    pub fn add_result(&mut self, result: TestResult) {
        self.test_results.push(result);
    }
    
    /// 添加多个测试结果
    pub fn add_results(&mut self, results: Vec<TestResult>) {
        self.test_results.extend(results);
    }
    
    /// 添加元数据
    pub fn add_metadata(&mut self, key: String, value: Value) {
        self.metadata.insert(key, value);
    }
    
    /// 生成报告
    pub fn generate_report(&self, format: ReportFormat) -> Result<GeneratedReport> {
        let content = match format {
            ReportFormat::Html => self.generate_html_report(),
            ReportFormat::Json => self.generate_json_report(),
            ReportFormat::JUnit => self.generate_junit_report(),
            ReportFormat::Console => self.generate_console_report(),
        }?;
        
        Ok(GeneratedReport {
            format,
            content,
            generated_at: self.generated_at,
        })
    }
    
    /// 生成 HTML 报告
    fn generate_html_report(&self) -> Result<String> {
        let summary = self.calculate_summary();
        
        let html = format!(
            include_str!("../../templates/test_report.html"),
            project_name = self.project_name,
            generated_at = self.generated_at.format("%Y-%m-%d %H:%M:%S UTC"),
            total_tests = summary.total_tests,
            passed_tests = summary.passed_tests,
            failed_tests = summary.failed_tests,
            skipped_tests = summary.skipped_tests,
            success_rate = summary.success_rate,
            total_duration = format_duration(summary.total_duration),
            test_results_html = self.generate_test_results_html(),
            charts_html = self.generate_charts_html(),
            metadata_html = self.generate_metadata_html(),
        );
        
        Ok(html)
    }
    
    /// 生成 JSON 报告
    fn generate_json_report(&self) -> Result<String> {
        let summary = self.calculate_summary();
        
        let report = serde_json::json!({
            "project": self.project_name,
            "generated_at": self.generated_at.to_rfc3339(),
            "summary": summary,
            "metadata": self.metadata,
            "results": self.test_results,
        });
        
        Ok(serde_json::to_string_pretty(&report)?)
    }
    
    /// 生成 JUnit XML 报告
    fn generate_junit_report(&self) -> Result<String> {
        let summary = self.calculate_summary();
        
        let mut xml = String::new();
        xml.push_str(&format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<testsuites name="{}" tests="{}" failures="{}" errors="{}" time="{}" timestamp="{}">
"#,
            self.project_name,
            summary.total_tests,
            summary.failed_tests,
            summary.failed_tests, // 简化处理，将失败都作为 failure
            summary.total_duration.as_secs_f64(),
            self.generated_at.to_rfc3339()
        ));
        
        // 按测试套件分组
        let mut suites: HashMap<String, Vec<&TestResult>> = HashMap::new();
        for result in &self.test_results {
            let suite_name = result.suite_name.clone();
            suites.entry(suite_name).or_insert_with(Vec::new).push(result);
        }
        
        for (suite_name, results) in suites {
            let suite_summary = self.calculate_suite_summary(&results);
            
            xml.push_str(&format!(
                r#"    <testsuite name="{}" tests="{}" failures="{}" errors="{}" time="{}">
"#,
                suite_name,
                suite_summary.total_tests,
                suite_summary.failed_tests,
                suite_summary.failed_tests,
                suite_summary.total_duration.as_secs_f64()
            ));
            
            for result in results {
                xml.push_str(&format!(
                    r#"        <testcase name="{}" classname="{}" time="{}">
"#,
                    result.name,
                    result.suite_name,
                    result.duration.as_secs_f64()
                ));
                
                if !result.success {
                    xml.push_str(&format!(
                        r#"            <failure message="{}">
{}
            </failure>
"#,
                        result.error_message.as_deref().unwrap_or("Test failed"),
                        result.output.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")
                    ));
                }
                
                xml.push_str("        </testcase>\n");
            }
            
            xml.push_str("    </testsuite>\n");
        }
        
        xml.push_str("</testsuites>\n");
        
        Ok(xml)
    }
    
    /// 生成控制台报告
    fn generate_console_report(&self) -> Result<String> {
        let summary = self.calculate_summary();
        
        let mut report = String::new();
        
        // 标题
        report.push_str(&format!("🧪 {} 测试报告\n", self.project_name));
        report.push_str(&"=".repeat(50));
        report.push_str("\n\n");
        
        // 总结
        report.push_str("📊 测试总结\n");
        report.push_str(&"-".repeat(20));
        report.push_str("\n");
        report.push_str(&format!("总测试数: {}\n", summary.total_tests));
        report.push_str(&format!("通过: {}\n", summary.passed_tests));
        report.push_str(&format!("失败: {}\n", summary.failed_tests));
        report.push_str(&format!("跳过: {}\n", summary.skipped_tests));
        report.push_str(&format!("成功率: {:.1}%\n", summary.success_rate));
        report.push_str(&format!("总耗时: {}\n", format_duration(summary.total_duration)));
        report.push_str("\n");
        
        // 失败的测试
        if summary.failed_tests > 0 {
            report.push_str("❌ 失败的测试\n");
            report.push_str(&"-".repeat(20));
            report.push_str("\n");
            
            for result in &self.test_results {
                if !result.success {
                    report.push_str(&format!("  - {} ({})\n", result.name, result.suite_name));
                    if let Some(error) = &result.error_message {
                        report.push_str(&format!("    错误: {}\n", error));
                    }
                }
            }
            report.push_str("\n");
        }
        
        // 详细的测试结果
        report.push_str("📋 详细结果\n");
        report.push_str(&"-".repeat(20));
        report.push_str("\n");
        
        for result in &self.test_results {
            let status_icon = if result.success { "✅" } else { "❌" };
            report.push_str(&format!(
                "{} {} ({}) - {}\n",
                status_icon,
                result.name,
                result.suite_name,
                format_duration(result.duration)
            ));
            
            if !result.success && self.metadata.get("verbose").and_then(|v| v.as_bool()).unwrap_or(false) {
                if let Some(error) = &result.error_message {
                    report.push_str(&format!("    错误: {}\n", error));
                }
            }
        }
        
        // 元数据
        if !self.metadata.is_empty() {
            report.push_str("\n📝 元数据\n");
            report.push_str(&"-".repeat(20));
            report.push_str("\n");
            
            for (key, value) in &self.metadata {
                report.push_str(&format!("  {}: {}\n", key, value));
            }
        }
        
        Ok(report)
    }
    
    /// 生成测试结果 HTML
    fn generate_test_results_html(&self) -> String {
        let mut html = String::new();
        
        for result in &self.test_results {
            let status_class = if result.success { "success" } else { "failure" };
            let status_icon = if result.success { "✅" } else { "❌" };
            let status_text = if result.success { "通过" } else { "失败" };
            
            html.push_str(&format!(
                r#"<div class="test-result {}">
    <div class="test-header">
        <span class="status-icon">{}</span>
        <span class="test-name">{}</span>
        <span class="test-suite">{}</span>
        <span class="test-duration">{}</span>
    </div>
    <div class="test-details">
        <span class="status-text">{}</span>
        <span class="test-timestamp">{}</span>
    </div>
"#,
                status_class,
                status_icon,
                result.name,
                result.suite_name,
                format_duration(result.duration),
                status_text,
                result.timestamp.format("%Y-%m-%d %H:%M:%S")
            ));
            
            if !result.success {
                if let Some(error) = &result.error_message {
                    html.push_str(&format!(
                        r#"    <div class="test-error">
        <strong>错误:</strong> {}
    </div>
"#,
                        html_escape(error)
                    ));
                }
            }
            
            if !result.output.is_empty() {
                html.push_str(&format!(
                    r#"    <div class="test-output">
        <strong>输出:</strong>
        <pre>{}</pre>
    </div>
"#,
                    html_escape(&result.output)
                ));
            }
            
            html.push_str("</div>\n");
        }
        
        html
    }
    
    /// 生成图表 HTML
    fn generate_charts_html(&self) -> String {
        let summary = self.calculate_summary();
        
        format!(
            r#"<div class="charts">
    <div class="chart">
        <h3>测试结果分布</h3>
        <canvas id="pieChart"></canvas>
    </div>
    <div class="chart">
        <h3>执行时间分布</h3>
        <canvas id="barChart"></canvas>
    </div>
</div>

<script>
// 饼图数据
const pieCtx = document.getElementById('pieChart').getContext('2d');
new Chart(pieCtx, {{
    type: 'pie',
    data: {{
        labels: ['通过', '失败', '跳过'],
        datasets: [{{
            data: [{}, {}, {}],
            backgroundColor: ['#28a745', '#dc3545', '#ffc107']
        }}]
    }},
    options: {{
        responsive: true,
        plugins: {{
            legend: {{
                position: 'bottom'
            }}
        }}
    }}
}});

// 柱状图数据
const barCtx = document.getElementById('barChart').getContext('2d');
new Chart(barCtx, {{
    type: 'bar',
    data: {{
        labels: ['单元测试', '集成测试', 'UAT测试', 'BDD测试', '性能测试'],
        datasets: [{{
            label: '测试数量',
            data: [12, 8, 5, 3, 2],
            backgroundColor: '#007bff'
        }}]
    }},
    options: {{
        responsive: true,
        scales: {{
            y: {{
                beginAtZero: true
            }}
        }}
    }}
}});
</script>
"#,
            summary.passed_tests,
            summary.failed_tests,
            summary.skipped_tests
        )
    }
    
    /// 生成元数据 HTML
    fn generate_metadata_html(&self) -> String {
        let mut html = String::new();
        
        if !self.metadata.is_empty() {
            html.push_str(r#"<div class="metadata">
    <h3>元数据</h3>
    <table>
"#);
            
            for (key, value) in &self.metadata {
                html.push_str(&format!(
                    r#"        <tr>
            <td>{}</td>
            <td>{}</td>
        </tr>
"#,
                    key,
                    value
                ));
            }
            
            html.push_str("    </table>\n</div>\n");
        }
        
        html
    }
    
    /// 计算测试总结
    fn calculate_summary(&self) -> TestSummary {
        let total_tests = self.test_results.len();
        let passed_tests = self.test_results.iter().filter(|r| r.success).count();
        let failed_tests = self.test_results.iter().filter(|r| !r.success).count();
        let skipped_tests = self.test_results.iter().filter(|r| r.skipped).count();
        let success_rate = if total_tests > 0 {
            (passed_tests as f64 / total_tests as f64) * 100.0
        } else {
            0.0
        };
        let total_duration = self.test_results.iter()
            .map(|r| r.duration)
            .sum();
        
        TestSummary {
            total_tests,
            passed_tests,
            failed_tests,
            skipped_tests,
            success_rate,
            total_duration,
        }
    }
    
    /// 计算测试套件总结
    fn calculate_suite_summary(&self, results: &[&TestResult]) -> TestSummary {
        let total_tests = results.len();
        let passed_tests = results.iter().filter(|r| r.success).count();
        let failed_tests = results.iter().filter(|r| !r.success).count();
        let skipped_tests = results.iter().filter(|r| r.skipped).count();
        let success_rate = if total_tests > 0 {
            (passed_tests as f64 / total_tests as f64) * 100.0
        } else {
            0.0
        };
        let total_duration = results.iter()
            .map(|r| r.duration)
            .sum();
        
        TestSummary {
            total_tests,
            passed_tests,
            failed_tests,
            skipped_tests,
            success_rate,
            total_duration,
        }
    }
    
    /// 保存报告到文件
    pub async fn save_report(&self, format: ReportFormat, output_path: &Path) -> Result<()> {
        let report = self.generate_report(format)?;
        
        // 确保目录存在
        if let Some(parent) = output_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        tokio::fs::write(output_path, report).await?;
        
        Ok(())
    }
    
    /// 生成并保存多种格式的报告
    pub async fn save_all_reports(&self, output_dir: &Path) -> Result<Vec<PathBuf>> {
        let mut saved_files = Vec::new();
        
        let formats = vec![
            (ReportFormat::Html, "test_report.html"),
            (ReportFormat::Json, "test_report.json"),
            (ReportFormat::JUnit, "test_report.xml"),
            (ReportFormat::Console, "test_report.txt"),
        ];
        
        for (format, filename) in formats {
            let output_path = output_dir.join(filename);
            self.save_report(format, &output_path).await?;
            saved_files.push(output_path);
        }
        
        Ok(saved_files)
    }
}

impl Default for TestReportGenerator {
    fn default() -> Self {
        Self::new("rs_guard".to_string())
    }
}

/// 测试结果
#[derive(Debug, Clone)]
pub struct TestResult {
    pub name: String,
    pub suite_name: String,
    pub success: bool,
    pub duration: std::time::Duration,
    pub timestamp: DateTime<Utc>,
    pub output: String,
    pub error_message: Option<String>,
    pub skipped: bool,
}

impl TestResult {
    pub fn new(name: String, suite_name: String) -> Self {
        Self {
            name,
            suite_name,
            success: false,
            duration: std::time::Duration::from_secs(0),
            timestamp: Utc::now(),
            output: String::new(),
            error_message: None,
            skipped: false,
        }
    }
    
    pub fn success(mut self) -> Self {
        self.success = true;
        self
    }
    
    pub fn failed(mut self, error: String) -> Self {
        self.success = false;
        self.error_message = Some(error);
        self
    }
    
    pub fn skipped(mut self) -> Self {
        self.skipped = true;
        self
    }
    
    pub fn with_duration(mut self, duration: std::time::Duration) -> Self {
        self.duration = duration;
        self
    }
    
    pub fn with_output(mut self, output: String) -> Self {
        self.output = output;
        self
    }
}

/// 测试总结
#[derive(Debug, Clone)]
pub struct TestSummary {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub skipped_tests: usize,
    pub success_rate: f64,
    pub total_duration: std::time::Duration,
}

/// 生成的报告
#[derive(Debug)]
pub struct GeneratedReport {
    pub format: ReportFormat,
    pub content: String,
    pub generated_at: DateTime<Utc>,
}

/// 辅助函数：格式化持续时间
fn format_duration(duration: std::time::Duration) -> String {
    if duration.as_secs() >= 60 {
        format!("{:.1}m", duration.as_secs_f64() / 60.0)
    } else if duration.as_millis() >= 1000 {
        format!("{:.1}s", duration.as_millis() as f64 / 1000.0)
    } else {
        format!("{}ms", duration.as_millis())
    }
}

/// 辅助函数：HTML 转义
fn html_escape(text: &str) -> String {
    text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")
}

/// 便捷函数：创建测试报告生成器
pub fn create_report_generator(project_name: &str) -> TestReportGenerator {
    TestReportGenerator::new(project_name.to_string())
}

/// 便捷宏：快速创建测试结果
#[macro_export]
macro_rules! test_result {
    ($name:expr, $suite:expr) => {
        $crate::TestResult::new($name.to_string(), $suite.to_string())
    };
}

#[macro_export]
macro_rules! test_success {
    ($name:expr, $suite:expr) => {
        $crate::test_result!($name, $suite).success()
    };
}

#[macro_export]
macro_rules! test_failure {
    ($name:expr, $suite:expr, $error:expr) => {
        $crate::test_result!($name, $suite).failed($error.to_string())
    };
}

#[macro_export]
macro_rules! test_skipped {
    ($name:expr, $suite:expr) => {
        $crate::test_result!($name, $suite).skipped()
    };
}