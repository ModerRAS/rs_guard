# Architecture Overview

`rs_guard` is designed as a monolithic repository (monorepo) containing a Rust workspace with multiple crates. This structure helps in sharing code, managing dependencies, and building the entire project cohesively.

## Workspace Crates

The workspace is composed of three main crates:

### 1. `backend`

This is the core of the service. It's a mixed-type crate, containing both a library (`src/lib.rs`) and a binary (`src/main.rs`).

-   **Library (`lib.rs`)**: Contains all business logic, including:
    -   Reed-Solomon encoding and decoding.
    -   File watching and integrity checking.
    -   Metadata database management.
    -   The Axum web server routing and API handlers.
-   **Binary (`main.rs`)**: A minimal entry point that simply calls the library's `run` function to start the application.

This separation ensures that the core logic is testable and can be easily integrated with other applications if needed.

### 2. `frontend`

This is a Yew-based WebAssembly (Wasm) application that provides a web interface for the user.

-   It communicates with the `backend` via a JSON REST API.
-   It's responsible for displaying status, logs, and providing buttons to trigger manual operations like integrity checks.
-   It's built using `trunk`, which simplifies the Wasm build process and provides a good development experience with hot-reloading.

### 3. `shared`

This is a simple library crate that contains data structures used for communication between the `backend` and `frontend`.

-   It primarily holds structs like `AppStatus` and enums like `ServiceStatus`.
-   By deriving `serde::Serialize` and `serde::Deserialize`, these structs can be easily converted to and from JSON.
-   Using a shared crate avoids code duplication and ensures type safety across the API boundary.

## Data Flow & Deployment

-   **Development**: The backend runs as a native binary, while the frontend is served by `trunk`'s development server, which proxies API requests to the backend.
-   **Production**: The `frontend` crate is built into a set of static assets (HTML, JS, Wasm). These assets are then embedded directly into the `backend` binary using `rust-embed`. The final product is a single, self-contained executable file, making deployment trivial. 