mod processor;
mod worker;
mod aggregator;

use processor::{start_file_processing, ProcessorConfig};
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <directory> <sleep_duration_secs>", args[0]);
        return;
    }

    let directory = &args[1];
    let sleep_duration_secs = args[2].parse::<u64>().unwrap_or_else(|_| {
        eprintln!("Invalid sleep duration, defaulting to 5 seconds.");
        5
    });

    println!(
        "Starting bulk file processor for directory: {} with a scan interval of {} seconds",
        directory, sleep_duration_secs
    );

    let config = Arc::new(ProcessorConfig {
        sleep_duration_secs,
    });

    start_file_processing(directory, config).await;
}
