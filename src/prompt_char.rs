use std::env;
use colored::*;

pub fn get_char() -> colored::ColoredString {
    let user_char = env::var("PROMPT_CHAR").unwrap_or("$".into());
    let root_char = env::var("PROMPT_CHAR_ROOT").unwrap_or("#".into());
    let user_char_color = env::var("PROMPT_CHAR_COLOR").unwrap_or("green ".into());
    let root_char_color = env::var("PROMPT_CHAR_ROOT_COLOR").unwrap_or("red".into());

    let euid = unsafe { libc::geteuid() };
    match euid {
        0 => return root_char.color(root_char_color),
        _ => return user_char.color(user_char_color)
    }
}

