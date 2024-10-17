use chrono::prelude::*;

pub fn log(message: &str) {
    let utc: DateTime<Utc> = Utc::now();
    eprintln!("[{utc}]: {message}");
}

pub fn logstring(message: String) {
    log(&message.as_str());
}
