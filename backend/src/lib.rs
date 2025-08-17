use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use axum::{
    extract::State,
    response::Json,
    routing::{get, post},
    Router, http::StatusCode,
};
use shared::AppStatus;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use anyhow::Result;
// 在 release 构建中启用静态资源嵌入
#[cfg(not(debug_assertions))]
use rust_embed::RustEmbed;

#[cfg(not(debug_assertions))]
#[derive(RustEmbed)]
#[folder = "../frontend/dist/"]
struct Assets;

pub mod checker;
pub mod config;
pub mod encoder;
pub mod metadata;
pub mod repair;
pub mod watcher;

// Define an application state that can be shared across handlers.
pub type AppState = Arc<Mutex<AppStatus>>;
pub type DbState = Arc<metadata::MetadataDb>;


pub async fn run() -> Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        )
        .init();

    // Load configuration
    let app_config = config::load_config("config/folders.toml")?;
    tracing::info!("Configuration loaded: {:?}", app_config);

    // Create shared application state
    let app_state = Arc::new(Mutex::new(AppStatus {
        watched_dirs: app_config.watched_directories.iter().map(|p| p.to_str().unwrap_or_default().to_string()).collect(),
        data_shards: app_config.data_shards,
        parity_shards: app_config.parity_shards,
        ..Default::default()
    }));

    // Open the metadata database
    // TODO: The DB path should be configurable.
    let db = Arc::new(metadata::open_db("rs_guard_meta.db")?);

    // Start file watcher
    let watcher_paths = app_config.watched_directories.clone();
    watcher::start_watching(app_state.clone(), watcher_paths)?;
    tracing::info!("File watcher started.");

    // TODO: Start a periodic background task for checking integrity.
    let state_clone = app_state.clone();
    let db_clone = db.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(3600)); // Check every hour
        loop {
            interval.tick().await;
            tracing::info!("Kicking off periodic integrity check.");
            if let Err(e) = checker::run_check(state_clone.clone(), db_clone.clone()).await {
                tracing::error!("Periodic check failed: {}", e);
            }
        }
    });
    
    let app = app_router(app_state, db);

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

pub fn app_router(app_state: AppState, db: DbState) -> Router {
     // Define API routes
    let api_router = Router::new()
        .route("/status", get(get_status))
        .route("/run-check", post(run_check_handler))
        .route("/run-repair", post(run_repair_handler))
        .with_state((app_state, db));

    // Conditionally serve static files based on build profile
    #[cfg(debug_assertions)]
    {
        // In debug builds, serve from the filesystem for hot-reloading
        Router::new()
            .nest("/api", api_router)
            .fallback_service(ServeDir::new("../frontend/dist").append_index_html_on_directories(true))
    }
    #[cfg(not(debug_assertions))]
    {
        // In release builds, serve from the embedded assets for a single-binary deployment
        Router::new()
            .nest("/api", api_router)
            .fallback_service(ServeDir::new("../frontend/dist").append_index_html_on_directories(true))
    }
}

pub async fn get_status(State((app_state, _db)): State<(AppState, DbState)>) -> Json<AppStatus> {
    let state = app_state.lock().unwrap().clone();
    Json(state)
}

async fn run_check_handler(State((app_state, db)): State<(AppState, DbState)>) -> StatusCode {
    tracing::info!("Manual integrity check triggered via API.");
    // Spawn a task to avoid blocking the API response
    tokio::spawn(async move {
        if let Err(e) = checker::run_check(app_state, db).await {
            tracing::error!("Manual check failed: {}", e);
        }
    });
    StatusCode::ACCEPTED
}

async fn run_repair_handler(State((app_state, db)): State<(AppState, DbState)>) -> StatusCode {
    tracing::info!("Manual repair triggered via API.");
    tokio::spawn(async move {
        if let Err(e) = repair::run_repair(app_state, db).await {
            tracing::error!("Manual repair failed: {}", e);
        }
    });
    StatusCode::ACCEPTED
} 