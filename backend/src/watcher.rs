use anyhow::Result;
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config};
use std::path::Path;
use std::sync::{Arc, Mutex};
use shared::AppStatus;
use std::time::Duration;

/// Spawns a background task to watch for file changes in the specified directories.
pub fn start_watching(app_status: Arc<Mutex<AppStatus>>, paths: Vec<impl AsRef<Path>>) -> Result<()> {
    
    let (tx, rx) = std::sync::mpsc::channel();

    // This watcher will run in its own thread, so we can't use async here directly.
    // Instead, it sends events back to our tokio runtime via a channel.
    let mut watcher = RecommendedWatcher::new(tx, Config::default()
        .with_poll_interval(Duration::from_secs(2)))?;

    for path in paths {
        watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
    }

    // This thread will block on receiving events and forward them to our main async runtime.
    tokio::spawn(async move {
        // TODO: This is a simplified receiver. A real implementation should:
        // 1. Handle different event types (Create, Remove, Modify).
        // 2. Batch events to avoid redundant processing (e.g., for large file copies).
        // 3. Trigger the encoding process for new/modified files.
        // 4. Trigger metadata updates for removed files.
        // 5. Log events to the AppStatus.
        for res in rx {
            match res {
                Ok(event) => {
                    println!("[Watcher] Event: {:?}", event);
                    let mut status = app_status.lock().unwrap();
                    status.logs.push(format!("[Watcher] Event: {:?}", event.kind));
                }
                Err(e) => eprintln!("[Watcher] Error: {:?}", e),
            }
        }
    });

    Ok(())
} 