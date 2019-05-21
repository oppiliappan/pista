use std::env;
use tico::tico;
use colored::*;

pub fn cwd() -> colored::ColoredString {
    let mut path = env::var("PWD").unwrap();
    let home = env::var("HOME").unwrap();
    let tilde_expand = env::var("EXPAND_TILDE").unwrap_or("0".into());

    match tilde_expand.as_ref() {
        "0" => path = path.replace(&home[..], "~"),
        _ => {}
    };

    let cwd_shorten = env::var("SHORTEN_CWD").unwrap_or("1".into());
    let cwd_color = env::var("CWD_COLOR").unwrap_or("white".into());
    match cwd_shorten.as_ref() {
        "0" => return path.color(cwd_color),
        _ => return tico(&path[..]).color(cwd_color)
    }

}
