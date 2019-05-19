use std::env;
use tico::tico;

fn main() {
    println!("{}", cwd());
    println!("{}", prompt_char());
}

fn cwd() -> String {
    let path = env::var("PWD").unwrap();
    let short_or_not = env::var("SHORTEN_CWD").unwrap_or("1".into());

    match short_or_not.as_ref() {
        "0" => path,
        _ => tico(&path[..])
    }
}

fn prompt_char() -> String {
    let user_char = env::var("PROMPT_CHAR").unwrap_or("$ ".into());
    let root_char = env::var("PROMPT_CHAR_ROOT").unwrap_or("# ".into());

    let euid = unsafe { libc::geteuid() };
    match euid {
        0 => return root_char,
        _ => return user_char
    }
}

fn vcs() -> String {
}
