
use std::io::Write;

const DEBUG: bool = true;

pub fn print(msg: String) {
    if DEBUG{
        let nmsg: String = msg + &String::from("\n");
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("log"){

            file.write(nmsg.as_bytes());
        }
    }
}

pub fn print_error<E: std::error::Error>(err: E) {
    if DEBUG{
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("debug_log"){

            let var_name = err.to_string();
            let msg: String = var_name + &String::from("\n");
            file.write(msg.as_bytes());
        }
    }
}

