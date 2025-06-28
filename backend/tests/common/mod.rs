use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use shared::AppStatus;
use backend::{config, metadata, app_router};
use tokio::net::TcpListener;

/// A helper function to spawn a test server in the background.
/// Returns the address the server is listening on.
pub async fn spawn_app() -> SocketAddr {
    // Bind to a random available port using tokio's listener
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");
    let addr = listener.local_addr().unwrap();

    // Use a mock configuration for testing
    let app_config = config::AppConfig {
        watched_directories: vec!["../test-data/source".into()], // Use a valid relative path from test execution dir
        data_shards: 4,
        parity_shards: 2,
    };
    
    // Create a temporary database for each test run to ensure isolation
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let db_path = temp_dir.path().join("test_db");
    let db = Arc::new(metadata::open_db(db_path.to_str().unwrap()).expect("Failed to open test DB"));

    let app_state = Arc::new(Mutex::new(AppStatus {
        watched_dirs: vec!["./test-data/source".to_string()],
        data_shards: app_config.data_shards,
        parity_shards: app_config.parity_shards,
        ..Default::default()
    }));

    // Build the app router using the function from our library
    let app = app_router(app_state, db);
    
    // Spawn the server in a background task
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    addr
} 