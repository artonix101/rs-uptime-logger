use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use chrono::Utc;
use users::{get_current_uid, get_user_by_uid};

const LOG_PATH: &str = "/var/log/rs-uptime-logger.log";

fn main() {
    //get user info
    let (user, uid) = get_requesting_user();

    //which command is being called? (shutdown, reboot, poweroff)
    let exe_path = env::args_os().next().unwrap();
    let exe_name = Path::new(&exe_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("shutdown");

    //path to the real system binary
    let real_bin = format!("/sbin/{}.real", exe_name);

    // Get command line arguments
    let args: Vec<_> = env::args_os().skip(1).collect();
    let cmdline = format!(
        "{} {}",
        exe_name,
        args.iter()
            .map(|a| a.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ")
    );

    //timestamp
    let timestamp = Utc::now().to_rfc3339();

    //log the shutdown/reboot request
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_PATH)
    {
        let _ = writeln!(
            file,
            "[{}] {} requested by user: {} (uid {})",
            timestamp, exe_name, user, uid
        );
        let _ = writeln!(file, "[{}] Command: {}", timestamp, cmdline.trim());
    }

    //delegate to the real binary
    let status = Command::new(&real_bin)
        .args(&args)
        .status()
        .expect(&format!("Failed to execute {}", real_bin));
    std::process::exit(status.code().unwrap_or(1));
}

// Find the real user who requested shutdown/reboot/poweroff
fn get_requesting_user() -> (String, u32) {
    // If run via sudo
    if let Ok(sudo_user) = env::var("SUDO_USER") {
        if let Some(user) = users::get_user_by_name(&sudo_user) {
            return (sudo_user, user.uid());
        } else {
            return (sudo_user, 0);
        }
    }
    // If run via pkexec
    if let Ok(pkexec_uid) = env::var("PKEXEC_UID") {
        if let Ok(uid) = pkexec_uid.parse::<u32>() {
            if let Some(user) = get_user_by_uid(uid) {
                return (user.name().to_string_lossy().into_owned(), uid);
            }
            return (uid.to_string(), uid);
        }
    }
    // Fallback: use current uid
    let uid = get_current_uid();
    let user = get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().into_owned())
        .unwrap_or(uid.to_string());
    (user, uid)
}