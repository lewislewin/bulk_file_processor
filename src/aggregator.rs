use async_channel::Receiver;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref LOG_FILE: Mutex<std::fs::File> = Mutex::new(
        OpenOptions::new()
            .append(true)
            .create(true)
            .open("results.log")
            .expect("Failed to open log file")
    );
}

pub async fn aggregate_results(rx: Receiver<String>) {
    let mut stats: HashMap<String, usize> = HashMap::new();

    while let Ok(result) = rx.recv().await {
        log_to_file(&result);

        let entry = stats.entry("Processed Files".to_string()).or_insert(0);
        *entry += 1;

        println!("Aggregator received: {}", result);
        println!("Stats: {:?}", stats);
    }
}

fn log_to_file(message: &str) {
    if let Ok(mut file) = LOG_FILE.lock() {
        writeln!(file, "{}", message).expect("Failed to write to log file");
    } else {
        eprintln!("Failed to acquire log file lock");
    }
}
