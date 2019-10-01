use colored::*;
use kube_conf::Config;

pub fn get_context() -> colored::ColoredString {
    let config = Config::load_default();
    if !config.is_ok() {
        return "".white();
    }
    match config.unwrap().current_context {
        Some(context) => {
            let parts: Vec<&str> = context.split("/").collect();
            if parts.len() == 3 {
                // openshift context
                format!("{} {}/{}", "☸".blue(), parts[1].bright_black(), parts[0]).blue()
            } else {
                format!("{} {}", "☸".blue(), context).blue()
            }
        }
        _ => "nocontext".white(),
    }
}
