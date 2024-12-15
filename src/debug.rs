
use std::io::Write;

const DEBUG: bool = true;

pub fn print(msg: String) {
    if DEBUG{
        if let Ok(file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("log"){

            file.write(b"{msg}");
        }
    }
}

pub fn print_error<E: std::error::Error>(err: E) {
    if DEBUG{
        if let Ok(file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("debug_log"){

            let msg = err.to_string();
            file.write(b"{msg}");
        }
    }
}

