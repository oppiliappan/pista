use std::env;
use colored::*;
use std::path::Path;

pub fn get_name() -> colored::ColoredString {
    match env::var("VIRTUAL_ENV") {
        Ok(venv_path) => {
            let venv_name = Path::new(&venv_path[..]).file_name();
            if let Some(name) = venv_name {
                if let Some(valid_name) = name.to_str() {
                    return format!("({})", valid_name).bright_black();
                }
            }
        }
        Err(_) => {}
    }
    return "".white()
}

