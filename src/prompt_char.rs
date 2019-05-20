use std::env;
use colored::*;

pub fn prompt_char() -> colored::ColoredString {
    let user_char = env::var("PROMPT_CHAR").unwrap_or("$ ".into());
    let root_char = env::var("PROMPT_CHAR_ROOT").unwrap_or("# ".into());

    let euid = unsafe { libc::geteuid() };
    match euid {
        0 => return root_char.red(),
        _ => return user_char.green()
    }
}

