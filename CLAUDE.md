# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

`rs_guard` 是一个使用 Rust 构建的现代化数据保护服务，提供块级冗余备份与完整性校验。项目采用单体仓库结构，包含三个核心 crate：backend（后端服务）、frontend（Yew Wasm 前端）和 shared（共享数据结构）。

## 常用命令

### 开发环境运行
```bash
# 启动后端服务（端口 3000）
cargo run -p backend

# 启动前端开发服务器（端口 8080，需要新终端）
cd frontend && trunk serve
```

### 构建生产版本
```bash
# 构建前端静态资源
cd frontend && trunk build --release

# 构建内嵌前端的后端（生成单个可执行文件）
cargo build -p backend --release
```

### 测试
```bash
# 运行所有测试
cargo test

# 运行后端集成测试
cargo test -p backend

# 运行特定测试
cargo test status_endpoint_returns_ok_and_correct_payload
```

### 代码检查
```bash
# 格式化代码
cargo fmt

# 检查代码（未配置 clippy）
cargo check
```

## 架构概览

### Workspace 结构
- **backend/**: 核心后端服务，包含 Reed-Solomon 编码、文件监控、元数据管理和 Web API
- **frontend/**: 基于 Yew 的 Wasm 前端应用
- **shared/**: 前后端共享的数据结构定义
- **config/**: 配置文件目录

### 核心模块（backend/src/）
- **lib.rs**: 主要业务逻辑，Web 服务器设置和应用状态管理
- **main.rs**: 简单的程序入口点
- **config.rs**: 配置文件加载和管理
- **encoder.rs**: Reed-Solomon 编码/解码实现
- **checker.rs**: 数据完整性校验逻辑
- **repair.rs**: 数据修复逻辑
- **watcher.rs**: 文件系统监控
- **metadata.rs**: 元数据数据库操作（使用 sled）

### 关键技术栈
- **后端**: axum (Web 框架), tokio (异步运行时), reed-solomon-erasure (纠删码), notify (文件监控), sled (嵌入式数据库)
- **前端**: yew (Wasm 框架), trunk (构建工具), reqwasm (HTTP 客户端)
- **部署**: rust-embed 用于生产环境中内嵌前端资源

### 配置文件
- **config/folders.toml**: 主要配置文件，定义监控目录和 Reed-Solomon 参数
  - `watched_directories`: 要保护的目录列表
  - `data_shards`: 数据分片数量（默认 4）
  - `parity_shards`: 校验分片数量（默认 2）

### 开发注意事项
1. 开发时需要先创建 `./test-data/source` 目录，否则后端启动会报错
2. 前端开发服务器会自动代理 API 请求到后端
3. 生产构建时前端资源会被内嵌到后端二进制文件中
4. 使用 `tracing` 进行日志记录，支持环境变量配置日志级别

### 测试策略
- 集成测试位于 `backend/tests/` 目录
- 测试使用 `tokio::test` 进行异步测试
- 包含 API 端点测试，验证状态端点返回正确的配置信息