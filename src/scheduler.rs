use crate::time::Time24;

use std::process::Command;

use fltk::dialog::message;

#[allow(dead_code)]
pub fn shutdown_schedule(time: &Time24) {
    let mut time_now = Time24::new();
    time_now.now();

    let mut time_str = time.to_str();
    
    #[cfg(target_os = "linux")]
    if time.eq(&time_now) { time_str = "0".to_string() };
    let schedule_cmd = Command::new("shutdown")
        .arg("-h")
        .arg(&time_str)
        .spawn();

    #[cfg(target_os = "windows")]
    if time.eq(&time_now) { time_str = "-t 0".to_sring() };
    let schedule_cmd = Command::new("shutdown")
        .arg("-s")
        .arg(&time_str)
        .spawn();

    match schedule_cmd {
        Ok(_) => {} ,
        Err(e) => {
            let msg = "Error: ".to_string() + &e.to_string();
            message(300, 300, &msg);
        },
    }
}
