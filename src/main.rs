use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use signal_hook::consts::signal::*;
use signal_hook::flag;

fn log_event(event: &str) {
    use chrono::Local;
    use std::fs::OpenOptions;
    use std::io::Write;

    let timestamp = Local::now().to_rfc3339(); //get local time
    let log_line = format!("[{}] {}\n", timestamp, event); //build string
    let mut file = OpenOptions::new() //open log file for writing
        .create(true)
        .append(true)
        .open("/var/log/rs-uptime-logger.log")
        .expect("Failed to open log file");
    file.write_all(log_line.as_bytes()).expect("Failed to write log"); //write log-line to file
}

fn just_booted() -> bool {
    if let Ok(uptime_contents) = std::fs::read_to_string("/proc/uptime") {
        if let Some(first) = uptime_contents.split_whitespace().next() {
            if let Ok(seconds) = first.parse::<f64>() {
                return seconds < 60.0; // System booted less than 60 seconds ago
            }
        }
    }
    false
}

fn main() {
    if just_booted() {
        log_event("System boot detected");
    }

    let running = Arc::new(AtomicBool::new(true)); //ceates an atomic boolean flag wrapped in an atomic reference-counted pointer
    
    // Register SIGTERM and SIGINT handlers to clear running flag
    flag::register(SIGTERM, Arc::clone(&running)).expect("Failed to register SIGTERM handler");
    flag::register(SIGINT, Arc::clone(&running)).expect("Failed to register SIGINT handler");

    // Wait loop: just wait until running == false (signal received)
    while running.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_millis(200));
    }

    if just_booted() {
        log_event("System shutdown detected");
    }
}
