use chrono::Utc;
use std::fs;
use std::io::Read;

fn main() {
    // Check system uptime
    if !just_booted() {
        // Don't log if not actual boot
        return;
    }

    let now = Utc::now();
    let timestamp = now.to_rfc3339();

    if let Err(e) = fs::write("/home/ab/tmp/boot_time.txt", timestamp) { //todo fix path, /var/tmp doenst work bc. of permissions
        eprintln!("Failed to write boot time: {}", e);
    } else {
        log_event("System boot detected");
    }
}

fn log_event(event: &str) {
    use std::fs::OpenOptions; //flexible file opening
    use std::io::Write; //access to .write_all()

    let timestamp = Utc::now().to_rfc3339(); //get local time
    let log_line = format!("[{}] {}\n", timestamp, event); //build log entry string
    let mut file = OpenOptions::new() //open log file for writing
        .create(true)
        .append(true)
        .open("/var/log/rs-uptime-logger.log")
        .expect("Failed to open log file");
    file.write_all(log_line.as_bytes()).expect("Failed to write log"); //write log-line to file
}

// to prevent from logging in any event that is not a boot
fn just_booted() -> bool { //has system been up for less than a minute?
    // Read system uptime from /proc/uptime
    let mut contents = String::new();
    if let Ok(mut file) = fs::File::open("/proc/uptime") {
        if file.read_to_string(&mut contents).is_ok() {
            if let Some(uptime_str) = contents.split_whitespace().next() {
                if let Ok(uptime) = uptime_str.parse::<f64>() {
                    // Consider boot if uptime < 60 seconds
                    return uptime < 60.0;
                }
            }
        }
    }
    false // fallback: not boot event
}