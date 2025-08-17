//! 测试数据定义
//! 
//! 这个模块定义了各种测试用的固定数据。

/// 测试文件内容
pub struct TestFileContents;

impl TestFileContents {
    /// 基础文本内容
    pub const BASIC_TEXT: &str = "This is a basic test file content for rs_guard testing purposes.";
    
    /// 中等长度文本内容
    pub const MEDIUM_TEXT: &str = "This is a medium length test file content. \
        It contains multiple sentences and paragraphs to test various scenarios. \
        The content is designed to be representative of typical text files that might be protected.";
    
    /// 长文本内容
    pub const LONG_TEXT: &str = "This is a long test file content that spans multiple paragraphs. \
        It is designed to test the system's ability to handle larger files efficiently. \
        The content includes various characters and patterns to ensure comprehensive testing. \
        Each paragraph adds more content to simulate real-world usage scenarios. \
        This helps validate that the rs_guard system can handle files of substantial size \
        without performance degradation or errors. The content continues to expand \
        to provide adequate testing material for the protection mechanisms.";
    
    /// JSON 内容
    pub const JSON_CONTENT: &str = r#"
{
    "id": "test-file-001",
    "name": "Test File",
    "type": "text",
    "size": 1024,
    "created": "2024-01-01T00:00:00Z",
    "modified": "2024-01-01T00:00:00Z",
    "metadata": {
        "author": "Test System",
        "version": "1.0",
        "tags": ["test", "sample", "demo"]
    }
}"#;
    
    /// CSV 内容
    pub const CSV_CONTENT: &str = r#"id,name,category,value,status
1,Item 1,Category A,100,active
2,Item 2,Category B,200,inactive
3,Item 3,Category C,300,active
4,Item 4,Category A,150,inactive
5,Item 5,Category B,250,active"#;
    
    /// XML 内容
    pub const XML_CONTENT: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<root>
    <item id="1">
        <name>Test Item 1</name>
        <description>First test item</description>
        <value>100</value>
    </item>
    <item id="2">
        <name>Test Item 2</name>
        <description>Second test item</description>
        <value>200</value>
    </item>
</root>"#;
    
    /// 二进制数据
    pub const BINARY_DATA: &[u8] = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]; // PNG header
    
    /// Unicode 内容
    pub const UNICODE_CONTENT: &str = "Unicode test: 中文 Español Français Deutsch 日本語 한국어 🚀 🌟 💻 🔒";
    
    /// 特殊字符内容
    pub const SPECIAL_CHARS: &str = "Special chars: !@#$%^&*()_+-=[]{}|;':\",./<>?\\`~";
    
    /// 空内容
    pub const EMPTY_CONTENT: &str = "";
    
    /// 仅换行符内容
    pub const NEWLINES_ONLY: &str = "\n\n\n\n\n";
    
    /// 仅空格内容
    pub const SPACES_ONLY: &str = "     ";
}

/// 测试文件名称
pub struct TestFileNames;

impl TestFileNames {
    pub const BASIC_TXT: &str = "basic.txt";
    pub const MEDIUM_TXT: &str = "medium.txt";
    pub const LARGE_TXT: &str = "large.txt";
    pub const JSON_FILE: &str = "data.json";
    pub const CSV_FILE: &str = "data.csv";
    pub const XML_FILE: &str = "data.xml";
    pub const BINARY_FILE: &str = "binary.bin";
    pub const UNICODE_FILE: &str = "unicode.txt";
    pub const SPECIAL_FILE: &str = "special.txt";
    pub const EMPTY_FILE: &str = "empty.txt";
    pub const NEWLINES_FILE: &str = "newlines.txt";
    pub const SPACES_FILE: &str = "spaces.txt";
    pub const CONFIG_FILE: &str = "config.toml";
    pub const LOG_FILE: &str = "app.log";
    pub const BACKUP_FILE: &str = "backup.tar.gz";
}

/// 测试目录名称
pub struct TestDirNames;

impl TestDirNames {
    pub const SOURCE: &str = "source";
    pub const BACKUP: &str = "backup";
    pub const TEMP: &str = "temp";
    pub const LOGS: &str = "logs";
    pub const CONFIG: &str = "config";
    pub const DATA: &str = "data";
    pub const DOCUMENTS: &str = "documents";
    pub const IMAGES: &str = "images";
    pub const VIDEOS: &str = "videos";
    pub const MUSIC: &str = "music";
}

/// 测试配置数据
pub struct TestConfigData;

impl TestConfigData {
    /// 基础配置
    pub const BASIC_CONFIG: &str = r#"
watched_directories = ["./test-data/source"]
data_shards = 4
parity_shards = 2
"#;
    
    /// 高级配置
    pub const ADVANCED_CONFIG: &str = r#"
watched_directories = [
    "./test-data/source",
    "./test-data/documents",
    "./test-data/images"
]
data_shards = 6
parity_shards = 3
"#;
    
    /// 最小配置
    pub const MINIMAL_CONFIG: &str = r#"
watched_directories = ["./source"]
data_shards = 2
parity_shards = 1
"#;
    
    /// 无效配置（缺少必需字段）
    pub const INVALID_CONFIG: &str = r#"
data_shards = 4
parity_shards = 2
"#;
}

/// 测试场景数据
pub struct TestScenarioData;

impl TestScenarioData {
    /// 单文件保护场景
    pub const SINGLE_FILE_PROTECTION: &str = "single_file_protection";
    
    /// 多文件保护场景
    pub const MULTIPLE_FILES_PROTECTION: &str = "multiple_files_protection";
    
    /// 大文件处理场景
    pub const LARGE_FILE_PROCESSING: &str = "large_file_processing";
    
    /// 文件更新场景
    pub const FILE_UPDATE_SCENARIO: &str = "file_update_scenario";
    
    /// 文件删除场景
    pub const FILE_DELETION_SCENARIO: &str = "file_deletion_scenario";
    
    /// 并发文件操作场景
    pub const CONCURRENT_OPERATIONS: &str = "concurrent_operations";
    
    /// 错误处理场景
    pub const ERROR_HANDLING: &str = "error_handling";
    
    /// 性能测试场景
    pub const PERFORMANCE_TESTING: &str = "performance_testing";
}

/// 测试元数据
pub struct TestMetadata;

impl TestMetadata {
    /// 获取测试文件元数据
    pub fn test_file_metadata() -> Vec<TestFileMeta> {
        vec![
            TestFileMeta {
                name: TestFileNames::BASIC_TXT.to_string(),
                size: TestFileContents::BASIC_TEXT.len(),
                content_type: "text/plain".to_string(),
                checksum: calculate_checksum(TestFileContents::BASIC_TEXT.as_bytes()),
            },
            TestFileMeta {
                name: TestFileNames::JSON_FILE.to_string(),
                size: TestFileContents::JSON_CONTENT.len(),
                content_type: "application/json".to_string(),
                checksum: calculate_checksum(TestFileContents::JSON_CONTENT.as_bytes()),
            },
            TestFileMeta {
                name: TestFileNames::BINARY_FILE.to_string(),
                size: TestFileContents::BINARY_DATA.len(),
                content_type: "application/octet-stream".to_string(),
                checksum: calculate_checksum(TestFileContents::BINARY_DATA),
            },
        ]
    }
}

/// 测试文件元数据
#[derive(Debug, Clone)]
pub struct TestFileMeta {
    pub name: String,
    pub size: usize,
    pub content_type: String,
    pub checksum: String,
}

/// 计算校验和（简化版本）
fn calculate_checksum(data: &[u8]) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// 测试文件集合
pub struct TestFileCollections;

impl TestFileCollections {
    /// 小文件集合
    pub fn small_files() -> Vec<(&'static str, &'static str)> {
        vec![
            (TestFileNames::BASIC_TXT, TestFileContents::BASIC_TEXT),
            (TestFileNames::EMPTY_FILE, TestFileContents::EMPTY_CONTENT),
            (TestFileNames::NEWLINES_FILE, TestFileContents::NEWLINES_ONLY),
        ]
    }
    
    /// 中等文件集合
    pub fn medium_files() -> Vec<(&'static str, &'static str)> {
        vec![
            (TestFileNames::MEDIUM_TXT, TestFileContents::MEDIUM_TEXT),
            (TestFileNames::JSON_FILE, TestFileContents::JSON_CONTENT),
            (TestFileNames::CSV_FILE, TestFileContents::CSV_CONTENT),
            (TestFileNames::XML_FILE, TestFileContents::XML_CONTENT),
        ]
    }
    
    /// 大文件集合
    pub fn large_files() -> Vec<(&'static str, String)> {
        vec![
            (TestFileNames::LARGE_TXT, TestFileContents::LONG_TEXT.repeat(10)),
            (TestFileNames::UNICODE_FILE, TestFileContents::UNICODE_CONTENT.repeat(5)),
            (TestFileNames::SPECIAL_FILE, TestFileContents::SPECIAL_CHARS.repeat(20)),
        ]
    }
    
    /// 特殊文件集合
    pub fn special_files() -> Vec<(&'static str, Vec<u8>)> {
        vec![
            (TestFileNames::BINARY_FILE, TestFileContents::BINARY_DATA.to_vec()),
            (TestFileNames::SPACES_FILE, TestFileContents::SPACES_ONLY.as_bytes().to_vec()),
        ]
    }
    
    /// 所有测试文件
    pub fn all_files() -> Vec<TestFileSpec> {
        let mut files = Vec::new();
        
        // 添加文本文件
        for (name, content) in Self::small_files() {
            files.push(TestFileSpec::Text {
                name: name.to_string(),
                content: content.to_string(),
            });
        }
        
        for (name, content) in Self::medium_files() {
            files.push(TestFileSpec::Text {
                name: name.to_string(),
                content: content.to_string(),
            });
        }
        
        for (name, content) in Self::large_files() {
            files.push(TestFileSpec::Text {
                name: name.to_string(),
                content,
            });
        }
        
        // 添加二进制文件
        for (name, content) in Self::special_files() {
            files.push(TestFileSpec::Binary {
                name: name.to_string(),
                content,
            });
        }
        
        files
    }
}

/// 测试文件规格
#[derive(Debug, Clone)]
pub enum TestFileSpec {
    Text { name: String, content: String },
    Binary { name: String, content: Vec<u8> },
}

impl TestFileSpec {
    pub fn name(&self) -> &str {
        match self {
            TestFileSpec::Text { name, .. } => name,
            TestFileSpec::Binary { name, .. } => name,
        }
    }
    
    pub fn size(&self) -> usize {
        match self {
            TestFileSpec::Text { content, .. } => content.len(),
            TestFileSpec::Binary { content, .. } => content.len(),
        }
    }
    
    pub fn is_text(&self) -> bool {
        matches!(self, TestFileSpec::Text { .. })
    }
    
    pub fn is_binary(&self) -> bool {
        matches!(self, TestFileSpec::Binary { .. })
    }
}