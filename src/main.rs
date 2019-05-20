use std::env;
use tico::tico;
use git2::{ Repository, Status };
use colored::*;

fn main() {
    print!("{}", cwd());
    let (branch, status) = match vcs_status() {
        Some((x, y)) => {
            (x, y)
        },
        None => ("".into(), "".into())
    };
    println!(" {} {}", branch, status.dimmed());
    print!("{} ", prompt_char());
}

fn cwd() -> colored::ColoredString {
    let mut path = env::var("PWD").unwrap();
    let home = env::var("HOME").unwrap();
    let tilde_expand = env::var("EXPAND_TILDE").unwrap_or("0".into());

    match tilde_expand.as_ref() {
        "0" => {},
        _ => path = path.replace(&home[..], "~")
    };

    let cwd_shorten = env::var("SHORTEN_CWD").unwrap_or("1".into());
    let cwd_color = env::var("CWD_COLOR").unwrap_or("white".into());
    match cwd_shorten.as_ref() {
        "0" => return path.color(cwd_color),
        _ => return tico(&path[..]).color(cwd_color)
    }

}

fn prompt_char() -> colored::ColoredString {
    let user_char = env::var("PROMPT_CHAR").unwrap_or("$ ".into());
    let root_char = env::var("PROMPT_CHAR_ROOT").unwrap_or("# ".into());

    let euid = unsafe { libc::geteuid() };
    match euid {
        0 => return root_char.red(),
        _ => return user_char.green()
    }
}

fn vcs_status() -> Option<(colored::ColoredString, String)> {
    let current_dir = env::var("PWD").unwrap();

    let repo = match Repository::open(current_dir) {
        Ok(r) => r,
        Err(_) => return None
    };

    let reference = repo.head().unwrap();
    let mut branch;

    if reference.is_branch() {
        branch = format!("{}", reference.shorthand().unwrap()).green();
    } else {
        let commit = reference.peel_to_commit().unwrap();
        let id = commit.id();
        branch = format!("{:.6}", id).yellow();
    }

    let mut repo_stat = String::new();

    let file_stats = repo.statuses(None).unwrap();
    for file in file_stats.iter() {
        match file.status() {
            Status::WT_NEW |
                Status::WT_MODIFIED |
                Status::WT_DELETED |
                Status::WT_TYPECHANGE |
                Status::WT_RENAMED |
                Status::INDEX_NEW |
                Status::INDEX_MODIFIED |
                Status::INDEX_DELETED |
                Status::INDEX_TYPECHANGE |
                Status::INDEX_RENAMED => {
                    let stat_char = env::var("GIT_DIRTY").unwrap_or("×".into());
                    repo_stat = stat_char;
                    break;
                },
            _ => {
                let stat_char = env::var("GIT_CLEAN").unwrap_or("·".into());
                repo_stat = stat_char;
            }
        }
    }
    return Some((branch, repo_stat))
}
