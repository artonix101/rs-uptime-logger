use chrono::Local;
use std::fs;

fn main() {
    let now = Local::now();
    let timestamp = now.to_rfc3339();

    if let Err(e) = fs::write("/home/ab/tmp/boot_time.txt", timestamp) {
        eprintln!("Failed to write boot time: {}", e);
    } else {
        log_event("System boot detected");
    }
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

// fn just_booted() -> bool { //has system been up for less than a minute?
//     if let Ok(uptime_contents) = std::fs::read_to_string("/proc/uptime") { //try reading /proc/uptime and bind
//         if let Some(first) = uptime_contents.split_whitespace().next() { //splits contents by whitespace and take first value
//             if let Ok(seconds) = first.parse::<f64>() { //converts first string into floating point number
//                 return seconds < 60.0; //if uptime is less than 60s, system likely just booted.
//             }
//         }
//     }
//     false
// }