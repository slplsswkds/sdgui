use crate::time::Time24;
use std::process::Command;

#[allow(dead_code)]
pub fn shutdown_schedule(time: &Time24) {
    let mut time_now = Time24::new();
    time_now.now();
    let time_str = {
        if time.eq(&time_now) {
            "now".to_string()
        } else {
            time.to_str()
        }
    };

    println!("arg is {}", time_str);

    #[cfg(target_os = "linux")]
    Command::new("shutdown")
        .arg("-h")
        .arg(time_str)
        .spawn()
        .expect("failed");

    #[cfg(target_os = "windows")]
    Command::new("shutdown")
        .arg("-s")
        .arg(time_str)
        .spawn()
        .expect("failed");
}
