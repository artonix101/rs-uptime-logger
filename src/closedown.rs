use std::fs;
use chrono::{DateTime, Local, Utc};

fn main() {
    // Read boot time
    let start_str = fs::read_to_string("/home/ab/tmp/boot_time.txt")
        .expect("Failed to read boot time");
    let start: DateTime<Utc> = DateTime::parse_from_rfc3339(&start_str)
        .expect("Invalid boot time format")
        .with_timezone(&Utc);


    let end = Utc::now();
    let uptime = end - start; //calculate the total uptime 
    let secs = uptime.num_seconds().max(0); //convert to seconds
    let hours = secs / 3600;
    let mins = (secs % 3600) / 60;
    let secs = secs % 60;

    log_event(&format!(
        "System shutdown detected (uptime: {:02}:{:02}:{:02})",
        hours, mins, secs
    ));
}

fn log_event(event: &str) {
    use std::fs::OpenOptions; //flexible file opening
    use std::io::Write; //access to .write_all()

    let timestamp = Local::now().to_rfc3339(); //get local time
    let log_line = format!("[{}] {}\n", timestamp, event); //build log entry string
    let mut file = OpenOptions::new() //open log file for writing
        .create(true)
        .append(true)
        .open("/var/log/rs-uptime-logger.log")
        .expect("Failed to open log file");
    file.write_all(log_line.as_bytes()).expect("Failed to write log"); //write log-line to file
}
