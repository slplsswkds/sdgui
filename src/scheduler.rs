use crate::time::Time24;

use std::process::Command;

use fltk::dialog::message;

#[allow(dead_code)]
pub fn shutdown_schedule(time: &Time24) {

    let time_str = time.to_minutes();
    
    #[cfg(target_os = "linux")]
    let schedule_cmd = Command::new("shutdown")
        .arg("-h")
        .arg("+".to_string() + &time_str.to_string())
        .spawn();

    #[cfg(target_os = "windows")]
    let schedule_cmd = Command::new("shutdown")
        .arg("-s")
        .arg("-t")
        .arg(time_str)
        .spawn();

    match schedule_cmd {
        Ok(_) => {} ,
        Err(error) => {
            let msg = "Error: ".to_string() + &error.to_string() + &"\nMaybe you haven't permission to use \"shutdown\"".to_string();
            message(300, 300, &msg);

            #[cfg(target_os = "linux")]
            let msg = "Copying \"/usr/sbin/shutdown\" to \"/usr/bin/\" maybe will solve this error".to_string();

            message(300, 300, &msg);
        },
    }
}
