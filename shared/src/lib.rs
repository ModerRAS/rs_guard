use serde::{Deserialize, Serialize};

/// Represents the overall status of the background service.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub enum ServiceStatus {
    #[default]
    Idle,
    Scanning,
    Checking,
    Repairing,
    Error(String),
}

/// A structure to hold the application's current state, sent to the frontend.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AppStatus {
    pub status: ServiceStatus,
    pub watched_dirs: Vec<String>,
    pub last_check_time: Option<String>,
    pub last_check_result: String,
    pub total_files: u64,
    pub protected_files: u64,
    pub data_shards: usize,
    pub parity_shards: usize,
    pub logs: Vec<String>,
}
