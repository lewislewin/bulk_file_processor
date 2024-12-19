use async_channel::Sender;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, Read};
use std::time::Instant;
use tokio::task;

pub async fn process_file(file_path: String, tx: Sender<String>) -> io::Result<()> {
    let start = Instant::now();

    match read_and_hash_file(file_path.clone()).await {
        Ok(hash) => {
            let result = format!("Processed {} (SHA256: {})", file_path, hash);

            tx.send(result).await.map_err(|_| {
                io::Error::new(io::ErrorKind::Other, "Failed to send result to aggregator")
            })?;
        }
        Err(e) => {
            let error_message = format!("Failed to process {}: {}", file_path, e);
            tx.send(error_message).await.map_err(|_| {
                io::Error::new(io::ErrorKind::Other, "Failed to send error to aggregator")
            })?;
        }
    }

    println!("File {} processed in {:?}", file_path, start.elapsed());
    Ok(())
}

async fn read_and_hash_file(file_path: String) -> io::Result<String> {
    task::spawn_blocking(move || {
        let mut file = File::open(&file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let hash = Sha256::digest(&buffer);
        Ok(format!("{:x}", hash))
    })
    .await
    .unwrap()
}
