# rs_guard 🛡️

[![CI](https://github.com/ModerRAS/rs_guard/actions/workflows/ci.yml/badge.svg)](https://github.com/ModerRAS/rs_guard/actions/workflows/ci.yml)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![文档 (English)](https://img.shields.io/badge/doc-English-blue.svg)](./docs/README_en.md)

一个使用 Rust 构建的现代化数据保护服务，为您重要的目录提供持续的、块级别的冗余备份与完整性校验。

`rs_guard` 会监视您指定的文件，使用里德-所罗门码（Reed-Solomon）对其进行编码，并定期校验其数据完整性。当检测到数据损坏时，它会自动从冗余数据中恢复。所有这一切，都通过一个简洁、实时的 Web 界面进行管理。

![界面截图占位符](./docs/screenshot.png) 
*(注: 当 UI 开发更完善后，此处应替换为真实的界面截图)*

---

## ✨ 功能特性

-   **块级冗余备份**: 将文件分割成数据块，并使用里德-所罗门码生成校验块（可配置 `N+M` 冗余度）。
-   **持续完整性校验**: 在后台定时执行任务，周期性地验证原始文件和校验块的完整性。
-   **自动修复**: 如果某个数据块或校验块丢失、损坏，`rs_guard` 能够利用剩余的块自动重建它。
-   **实时文件监控**: 使用 `notify` 库监控文件系统，自动保护新增文件、更新被修改的文件。
-   **Web 管理界面**: 一个使用 Rust (Yew + Wasm) 构建的现代化、实时的仪表盘，用于监控服务状态、查看日志，并手动触发操作。
-   **单文件部署**: 在生产环境下，整个前端界面会被内嵌到后端二进制文件中，使部署过程简化到极致——只需拷贝一个文件。
-   **跨平台**: 设计为可以在 Windows 和 Linux 上作为长期运行的服务。

## 🛠️ 技术栈

| 领域     | 技术与工具                                                                                                 |
| :------- | :--------------------------------------------------------------------------------------------------------- |
| **后端** | [**`axum`**](https://crates.io/crates/axum) (Web 服务), [**`tokio`**](https://crates.io/crates/tokio) (异步运行时), [**`reed-solomon-erasure`**](https://crates.io/crates/reed-solomon-erasure) (纠删码), [**`notify`**](https://crates.io/crates/notify) (文件监控), [**`sled`**](https://crates.io/crates/sled) (元数据存储), [**`rust-embed`**](https://crates.io/crates/rust-embed) (内嵌前端资源)。 |
| **前端** | [**`yew`**](https://crates.io/crates/yew) (Wasm 响应式框架), [**`trunk`**](https://trunkrs.dev/) (构建与资源管理), [**`reqwasm`**](https://crates.io/crates/reqwasm) (API 请求)。 |
| **共享**   | [**`serde`**](https://crates.io/crates/serde) (用于在前后端之间进行可靠的序列化与反序列化)。                    |

如果希望深入了解项目结构，请查阅 [**架构概览**](./docs/architecture.md)。

## 🏗️ 系统架构

### 整体架构图

```mermaid
graph TB
    subgraph "用户界面层"
        UI[Web 浏览器]
        FRONTEND[Yew 前端应用<br/>(WebAssembly)]
    end
    
    subgraph "API 层"
        API[Axum Web 服务器<br/>端口 3000]
        STATIC[静态文件服务<br/>(开发模式: 文件系统<br/>生产模式: 内嵌资源)]
    end
    
    subgraph "业务逻辑层"
        CORE[核心服务引擎]
        WATCHER[文件监控器<br/>notify]
        ENCODER[Reed-Solomon 编码器<br/>reed-solomon-erasure]
        CHECKER[完整性检查器]
        REPAIR[数据修复器]
        SCHEDULER[任务调度器<br/>定时检查]
    end
    
    subgraph "数据存储层"
        METADATA[元数据数据库<br/>sled (嵌入式)]
        FILESYSTEM[文件系统<br/>原始文件 + 分片]
    end
    
    subgraph "配置层"
        CONFIG[配置管理器<br/>folders.toml]
    end
    
    UI --> FRONTEND
    FRONTEND --> API
    API --> CORE
    API --> STATIC
    
    CORE --> WATCHER
    CORE --> ENCODER
    CORE --> CHECKER
    CORE --> REPAIR
    CORE --> SCHEDULER
    
    WATCHER --> FILESYSTEM
    ENCODER --> FILESYSTEM
    CHECKER --> FILESYSTEM
    REPAIR --> FILESYSTEM
    
    CORE --> METADATA
    CHECKER --> METADATA
    REPAIR --> METADATA
    
    CORE --> CONFIG
    
    style UI fill:#e1f5fe
    style FRONTEND fill:#e8f5e8
    style API fill:#fff3e0
    style CORE fill:#f3e5f5
    style WATCHER fill:#e8eaf6
    style ENCODER fill:#e8eaf6
    style CHECKER fill:#e8eaf6
    style REPAIR fill:#e8eaf6
    style SCHEDULER fill:#e8eaf6
    style METADATA fill:#fce4ec
    style FILESYSTEM fill:#fce4ec
    style CONFIG fill:#f1f8e9
```

### 数据流程图

```mermaid
sequenceDiagram
    participant F as 文件系统
    participant W as Watcher
    participant E as Encoder
    participant M as Metadata
    participant C as Checker
    participant R as Repair
    participant S as 调度器
    
    Note over F,S: 文件保护流程
    F->>W: 文件创建/修改事件
    W->>E: 触发编码
    E->>F: 读取原始文件
    E->>E: Reed-Solomon 编码
    E->>F: 存储数据分片和校验分片
    E->>M: 存储元数据
    
    Note over F,S: 定时检查流程
    S->>C: 每小时触发检查
    C->>M: 获取文件列表
    C->>F: 验证文件完整性
    C->>F: 验证分片完整性
    alt 发现损坏
        C->>M: 标记需要修复
        C->>C: 记录损坏信息
    end
    
    Note over F,S: 自动修复流程
    S->>R: 触发修复
    R->>M: 获取损坏文件列表
    R->>F: 读取可用分片
    R->>R: Reed-Solomon 重构
    R->>F: 写入修复的分片/文件
    R->>M: 更新元数据状态
```

### Reed-Solomon 编码架构

```mermaid
graph LR
    subgraph "编码过程"
        A[原始文件<br/>100MB]
        B[数据分片器]
        C[Reed-Solomon<br/>编码引擎]
        D[数据分片存储<br/>4个 × 25MB]
        E[校验分片存储<br/>2个 × 25MB]
    end
    
    subgraph "解码/修复过程"
        F[可用分片<br/>任意 4+个]
        G[Reed-Solomon<br/>重构引擎]
        H[重构的原始文件<br/>100MB]
    end
    
    A --> B
    B --> C
    C --> D
    C --> E
    
    F --> G
    G --> H
    
    style A fill:#e3f2fd
    style B fill:#f3e5f5
    style C fill:#e8f5e8
    style D fill:#fff3e0
    style E fill:#ffebee
    style F fill:#fff3e0
    style G fill:#e8f5e8
    style H fill:#e3f2fd
```

### 组件交互详细图

```mermaid
graph TB
    subgraph "Backend 核心模块"
        LIB[lib.rs<br/>应用入口]
        MAIN[main.rs<br/>程序启动]
        CONFIG[config.rs<br/>配置管理]
        
        subgraph "核心服务"
            WATCHER[watcher.rs<br/>文件监控]
            ENCODER[encoder.rs<br/>编码/解码]
            CHECKER[checker.rs<br/>完整性检查]
            REPAIR[repair.rs<br/>数据修复]
            METADATA[metadata.rs<br/>元数据管理]
        end
        
        subgraph "Web 服务"
            ROUTER[路由管理]
            HANDLERS[API 处理器]
            STATIC[静态文件服务]
        end
    end
    
    subgraph "Frontend 模块"
        YEW[Yew 组件]
        API[API 客户端]
        STATE[状态管理]
    end
    
    subgraph "共享模块"
        SHARED[共享数据结构<br/>AppStatus, ServiceStatus]
    end
    
    MAIN --> LIB
    LIB --> CONFIG
    LIB --> WATCHER
    LIB --> ENCODER
    LIB --> CHECKER
    LIB --> REPAIR
    LIB --> METADATA
    LIB --> ROUTER
    
    ROUTER --> HANDLERS
    ROUTER --> STATIC
    
    HANDLERS --> SHARED
    YEW --> SHARED
    API --> SHARED
    
    HANDLERS --> WATCHER
    HANDLERS --> CHECKER
    HANDLERS --> REPAIR
    
    WATCHER --> METADATA
    ENCODER --> METADATA
    CHECKER --> METADATA
    REPAIR --> METADATA
    
    style LIB fill:#e8eaf6
    style MAIN fill:#e8eaf6
    style CONFIG fill:#f1f8e9
    style WATCHER fill:#fff3e0
    style ENCODER fill:#fff3e0
    style CHECKER fill:#fff3e0
    style REPAIR fill:#fff3e0
    style METADATA fill:#fff3e0
    style ROUTER fill:#e3f2fd
    style HANDLERS fill:#e3f2fd
    style STATIC fill:#e3f2fd
    style YEW fill:#e8f5e8
    style API fill:#e8f5e8
    style STATE fill:#e8f5e8
    style SHARED fill:#f3e5f5
```

## 🚀 快速上手

### 环境准备

1.  **安装 Rust**: 如果您尚未安装，请访问 [rustup.rs](https://rustup.rs/) 获取。
2.  **添加 Wasm 编译目标**: 前端需要编译到 WebAssembly。请通过此命令添加：
    ```bash
    rustup target add wasm32-unknown-unknown
    ```
3.  **安装 Trunk**: Trunk 是我们用于构建 Wasm 前端的工具。
    ```bash
    cargo install trunk
    ```

### 开发模式运行

为了获得最佳的开发体验（包含热重载），请在不同的终端中分别运行后端和前端。

1.  **运行后端服务**:
    ```bash
    # 此命令会监控文件变化，并在 http://127.0.0.1:3000 提供 API 服务
    cargo run -p backend
    ```
    *注意: 首次运行时，如果 `./test-data/source` 目录不存在，程序会报错退出。这是预期的行为，在您手动创建该目录后即可正常运行。*

2.  **运行前端服务**:
    ```bash
    # 此命令会在 http://127.0.0.1:8080 启动前端开发服务器，并自动代理 API 请求
    cd frontend
    trunk serve
    ```
    Trunk 会自动在您的浏览器中打开一个新标签页。您对前端代码的任何修改都会被自动编译并实时刷新到浏览器中。

## 📦 构建生产版本

要创建一个用于部署的、独立的二进制文件：

1.  **构建前端静态资源**:
    ```bash
    cd frontend
    trunk build --release
    ```
    此命令会在 `frontend/dist` 目录中生成优化后的静态文件。

2.  **构建内嵌前端的后端**:
    ```bash
    # 此命令会构建一个经过发布优化的二进制文件，并包含所有前端资源
    cargo build -p backend --release
    ```
    最终的可执行文件位于 `target/release/backend` (在 Windows 上是 `backend.exe`)。您只需将这一个文件拷贝到您的服务器上即可完成部署。

## 🤝 参与贡献

欢迎任何形式的贡献！无论是 Bug 报告、功能建议还是代码提交 (Pull Request)，都请随时参与。

1.  Fork 本仓库。
2.  创建您的功能分支 (`git checkout -b feature/AmazingFeature`)。
3.  提交您的修改 (`git commit -m 'Add some AmazingFeature'`)。
4.  将分支推送到远程 (`git push origin feature/AmazingFeature`)。
5.  创建一个 Pull Request。

## 📜 开源许可

本项目基于 **GNU General Public License v3.0** 许可。详情请参阅 [LICENSE](./LICENSE) 文件。 