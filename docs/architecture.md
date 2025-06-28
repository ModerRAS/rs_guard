# 架构概览

[English Version](./architecture_en.md)

`rs_guard` 被设计为一个单体仓库 (monorepo)，其中包含一个拥有多个 crate 的 Rust workspace。这种结构有助于代码共享、依赖管理，以及整个项目的一致性构建。

## Workspace Crates

工作区由三个核心的 crate 组成：

### 1. `backend` (后端)

这是整个服务的核心。它是一个混合类型的 crate，同时包含了库 (`src/lib.rs`) 和二进制入口 (`src/main.rs`)。

-   **库 (`lib.rs`)**: 包含所有业务逻辑，例如：
    -   里德-所罗门码的编码与解码。
    -   文件监控与完整性校验。
    -   元数据数据库管理。
    -   Axum Web 服务器的路由与 API 处理。
-   **二进制入口 (`main.rs`)**: 一个极简的程序入口，它只调用库中提供的 `run` 函数来启动应用。

这种“库优先”的分离设计确保了核心逻辑是可测试的，并且在需要时可以轻松地被其他应用集成。

### 2. `frontend` (前端)

这是一个基于 Yew 框架的 WebAssembly (Wasm) 应用，为用户提供可视化的 Web 操作界面。

-   它通过 JSON REST API 与 `backend` 进行通信。
-   它负责展示服务状态、运行日志，并提供按钮来触发手动的校验或修复操作。
-   它使用 `trunk` 进行构建，极大地简化了 Wasm 的构建流程，并提供了优秀的热重载开发体验。

### 3. `shared` (共享)

这是一个简单的库 crate，其中包含了 `backend` 和 `frontend` 之间通信所用的共享数据结构。

-   它主要定义了像 `AppStatus` (应用状态) 和 `ServiceStatus` (服务状态) 这样的结构体。
-   通过派生 `serde::Serialize` 和 `serde::Deserialize`，这些结构体可以轻松地在 JSON 和 Rust 类型之间转换。
-   使用共享 crate 避免了代码的重复，并确保了 API 边界两端的类型安全。

## 数据流与部署

-   **开发环境**: 后端作为一个本地二进制文件运行，而前端由 `trunk` 的开发服务器提供服务，该服务器会自动将 API 请求代理到后端。
-   **生产环境**: `frontend` crate 会被构建成一套静态资源 (HTML, JS, Wasm)。接着，这些资源会通过 `rust-embed` 被直接内嵌到 `backend` 的二进制文件中。最终的产物是一个独立的、自包含的可执行文件，使得部署过程极其简单。
