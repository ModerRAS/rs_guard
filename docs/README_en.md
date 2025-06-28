# rs_guard 🛡️

[![CI](https://github.com/ModerRAS/rs_guard/actions/workflows/ci.yml/badge.svg)](https://github.com/ModerRAS/rs_guard/actions/workflows/ci.yml)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

A modern data protection service built with Rust, providing continuous, block-level data redundancy and integrity checking for your important directories.

`rs_guard` watches your files, encodes them using Reed-Solomon erasure coding, and regularly verifies their integrity, automatically repairing them from redundant data if corruption is detected. All this is managed through a sleek, real-time web interface.

![Screenshot Placeholder](./docs/screenshot.png) 
*(Note: A real screenshot should be placed here once the UI is more developed)*

---

## ✨ Features

-   **Block-Level Redundancy**: Splits files into blocks and generates parity shards using Reed-Solomon encoding (configurable `N+M` redundancy).
-   **Continuous Integrity Checking**: Runs background tasks to periodically validate the integrity of both original files and their parity shards.
-   **Automatic Repair**: If a data block or shard is lost or corrupted, `rs_guard` can automatically reconstruct it from the remaining blocks.
-   **Live File Watching**: Uses `notify` to automatically protect new files and update redundancy for modified files.
-   **Web Interface**: A modern, real-time dashboard built with Rust (Yew + Wasm) to monitor status, view logs, and trigger manual operations.
-   **Single-File Deployment**: For production, the entire frontend is embedded into the backend binary, making deployment as simple as copying a single file.
-   **Cross-Platform**: Designed to run as a long-running service on both Windows and Linux.

## 🛠️ Tech Stack

| Area      | Technology                                                                                                  |
| :-------- | :---------------------------------------------------------------------------------------------------------- |
| **Backend** | [**`axum`**](https://crates.io/crates/axum) for the web server, [**`tokio`**](https://crates.io/crates/tokio) for async runtime, [**`reed-solomon-erasure`**](https://crates.io/crates/reed-solomon-erasure) for encoding, [**`notify`**](https://crates.io/crates/notify) for file watching, [**`sled`**](https://crates.io/crates/sled) for metadata storage, [**`rust-embed`**](https://crates.io/crates/rust-embed) for embedding the frontend. |
| **Frontend**  | [**`yew`**](https://crates.io/crates/yew) for the reactive Wasm framework, [**`trunk`**](https://trunkrs.dev/) for building and asset management, [**`reqwasm`**](https://crates.io/crates/reqwasm) for API requests. |
| **Shared**    | [**`serde`**](https://crates.io/crates/serde) for robust serialization between frontend and backend.              |

For a more detailed look at the project's structure, see the [**Architecture Overview**](./docs/architecture.md).

## 🚀 Getting Started

### Prerequisites

1.  **Install Rust**: If you don't have it, install from [rustup.rs](https://rustup.rs/).
2.  **Add Wasm Target**: The frontend compiles to WebAssembly. Add the target via:
    ```bash
    rustup target add wasm32-unknown-unknown
    ```
3.  **Install Trunk**: Trunk is our build tool for the Wasm frontend.
    ```bash
    cargo install trunk
    ```

### Running in Development

For the best development experience with hot-reloading, run the backend and frontend in separate terminals.

1.  **Run the Backend**:
    ```bash
    # This will watch for file changes and serve the API on http://127.0.0.1:3000
    cargo run -p backend
    ```
    *Note: The first time you run this, it will fail if the `./test-data/source` directory doesn't exist. This is expected. The directory will be created for subsequent runs.*

2.  **Run the Frontend**:
    ```bash
    # This serves the Yew app on http://127.0.0.1:8080 and proxies API requests
    cd frontend
    trunk serve
    ```
    Trunk will automatically open a browser tab. Any changes you make to the frontend code will be recompiled and reloaded in the browser automatically.

## 📦 Building for Production

To create a single, self-contained executable for deployment:

1.  **Build Frontend Assets**:
    ```bash
    cd frontend
    trunk build --release
    ```
    This generates optimized static files in the `frontend/dist` directory.

2.  **Build Backend with Embedded Frontend**:
    ```bash
    # This builds a release-optimized binary with all frontend assets included
    cargo build -p backend --release
    ```
    The final executable will be located at `target/release/backend` (or `backend.exe` on Windows). You can copy this single file to your server and run it.

## 🤝 Contributing

Contributions are welcome! Whether it's bug reports, feature suggestions, or pull requests, please feel free to engage.

1.  Fork the repository.
2.  Create your feature branch (`git checkout -b feature/AmazingFeature`).
3.  Commit your changes (`git commit -m 'Add some AmazingFeature'`).
4.  Push to the branch (`git push origin feature/AmazingFeature`).
5.  Open a Pull Request.

## 📜 License

This project is licensed under the **GNU General Public License v3.0**. See the [LICENSE](./LICENSE) file for details.