# rs_guard 测试指南

## 概述

rs_guard 项目使用多层次的测试策略来确保代码质量和系统稳定性。本指南说明了如何运行和理解各种测试。

## 测试层次

### 1. 单元测试
- **位置**: `backend/src/` 各个模块中的 `#[cfg(test)]` 块
- **目的**: 测试单个函数和模块的功能
- **运行**: `cargo test --lib`

### 2. 集成测试
- **位置**: `backend/tests/` 目录
- **目的**: 测试多个模块之间的交互
- **运行**: `cargo test integration_simple`

### 3. API 测试
- **位置**: `backend/tests/api_tests.rs`
- **目的**: 测试 HTTP API 端点
- **运行**: `cargo test api_tests`

### 4. BDD 测试
- **位置**: `tests/bdd_simple.sh`
- **目的**: 模拟用户操作，验证端到端功能
- **运行**: `./tests/bdd_simple.sh`

### 5. 用户验收测试 (UAT)
- **位置**: `tests/integration_simple.rs`
- **目的**: 验证系统是否满足用户需求
- **运行**: `cargo test integration_suite`

## 快速开始

### 1. 运行所有测试

```bash
# 使用提供的脚本运行所有测试
./run_tests.sh
```

### 2. 运行特定类型的测试

```bash
# 只运行单元测试
cargo test --lib

# 只运行集成测试
cargo test integration_simple

# 只运行 API 测试
cargo test api_tests

# 只运行 BDD 测试
./tests/bdd_simple.sh
```

### 3. 运行单个测试

```bash
# 运行特定的测试函数
cargo test test_service_status

# 运行特定模块的测试
cargo test backend::tests::status_endpoint_returns_ok_and_correct_payload
```

## 测试环境要求

### 必需的工具
- **Rust**: 1.70+
- **Cargo**: 包含在 Rust 中
- **curl**: 用于 HTTP 测试

### 可选的工具
- **jq**: 用于 JSON 处理
- **cargo-nextest**: 并行测试运行器
- **cargo-tarpaulin**: 代码覆盖率工具

### 测试数据
测试会自动创建和清理测试数据：
- `test-data/source/`: 测试源文件
- `test-data/output/`: 测试输出文件

## 测试覆盖的功能

### 文件保护功能
- ✅ 文件监控和检测
- ✅ Reed-Solomon 编码
- ✅ 数据完整性检查
- ✅ 自动数据修复

### Web 界面功能
- ✅ 服务状态查询
- ✅ 配置管理
- ✅ 日志查看
- ✅ 实时状态更新

### API 功能
- ✅ RESTful API 端点
- ✅ JSON 响应格式
- ✅ 错误处理
- ✅ 状态查询

### 性能和可靠性
- ✅ 并发操作
- ✅ 错误恢复
- ✅ 资源管理
- ✅ 内存使用

## 测试结果解读

### 成功的测试
```
✅ 服务状态检查通过
✅ Web 界面访问正常
✅ 文件操作测试通过
✅ 配置验证通过
✅ 健康检查通过
```

### 失败的测试
```
❌ 服务状态检查失败
❌ Web 界面访问失败
❌ 测试文件创建失败
❌ 配置文件不存在
❌ 健康检查失败
```

### 常见问题

1. **服务未启动**
   - 确保 `./target/release/backend` 正在运行
   - 检查端口 3000 是否被占用

2. **权限问题**
   - 确保有写入测试目录的权限
   - 检查文件系统权限

3. **依赖问题**
   - 运行 `cargo build` 确保所有依赖都已安装
   - 检查 Rust 版本兼容性

## 持续集成

### GitHub Actions
项目包含 GitHub Actions 工作流，会自动运行：
- 代码格式检查
- 单元测试
- 集成测试
- 构建验证

### 本地测试
在提交代码前，建议运行：
```bash
# 格式检查
cargo fmt --check

# 静态分析
cargo clippy -- -D warnings

# 运行所有测试
./run_tests.sh
```

## 编写新测试

### 添加单元测试
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        // 准备测试数据
        let input = "test";
        
        // 执行测试
        let result = my_function(input);
        
        // 验证结果
        assert_eq!(result, "expected");
    }
}
```

### 添加集成测试
```rust
use reqwest::Client;

#[tokio::test]
async fn test_api_endpoint() {
    let client = Client::new();
    let response = client
        .get("http://localhost:3000/api/status")
        .send()
        .await
        .unwrap();
    
    assert!(response.status().is_success());
}
```

### 添加 BDD 测试
```bash
# 在 bdd_simple.sh 中添加
echo "🧪 测试 6: 新功能测试"
echo "---------------------------------"
# 添加测试逻辑
```

## 测试报告

### 自动生成的报告
- `test_report.md`: 测试结果总结
- `backend/coverage/`: 代码覆盖率报告（如果安装了 tarpaulin）

### 自定义报告
可以通过修改 `run_tests.sh` 脚本来生成自定义格式的测试报告。

## 故障排除

### 测试失败时的调试步骤
1. 查看详细的测试输出：`cargo test -- --nocapture`
2. 检查测试环境：确保没有冲突的进程
3. 清理并重新构建：`cargo clean && cargo build`
4. 查看日志：`RUST_LOG=debug cargo test`

### 性能问题
- 使用 `cargo test --release` 运行优化版本
- 考虑减少并行测试线程数
- 增加测试超时时间

## 贡献指南

1. 新功能必须包含相应的测试
2. 测试覆盖率不应低于当前水平
3. 所有测试必须在 CI 环境中通过
4. 遵循现有的测试命名和结构约定

## 参考资料

- [Rust 测试文档](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Cargo 测试指南](https://doc.rust-lang.org/cargo/guide/tests.html)
- [BDD 最佳实践](https://cucumber.io/docs/bdd/)
- [API 测试最佳实践](https://martinfowler.com/articles/practical-test-pyramid.html)