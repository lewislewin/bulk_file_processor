use crate::worker::process_file;
use crate::aggregator::aggregate_results;
use async_channel::{unbounded, Sender};
use std::fs;
use std::path::Path;
use tokio::task;
use tokio::time::sleep;
use std::sync::Arc;

#[derive(Clone)]
pub struct ProcessorConfig {
    pub sleep_duration_secs: u64,
}

pub async fn start_file_processing(directory: &str, config: Arc<ProcessorConfig>) {
    let (tx, rx) = unbounded();

    let aggregator_handle = task::spawn(aggregate_results(rx));

    loop {
        if let Err(e) = scan_and_process_files(directory, tx.clone()).await {
            eprintln!("Error scanning directory: {}", e);
        }
        sleep(tokio::time::Duration::from_secs(config.sleep_duration_secs)).await;
    }

    let _ = aggregator_handle.await;
}

async fn scan_and_process_files(directory: &str, tx: Sender<String>) -> Result<(), std::io::Error> {
    let dir_path = Path::new(directory);

    if !dir_path.exists() || !dir_path.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Directory not found",
        ));
    }

    let entries = fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        let file_path = entry.path();

        if file_path.is_file() {
            let tx = tx.clone();
            let file_path = file_path.to_string_lossy().to_string();

            task::spawn(async move {
                if let Err(e) = process_file(file_path, tx).await {
                    eprintln!("Error processing file: {}", e);
                }
            });
        }
    }

    Ok(())
}
