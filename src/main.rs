use std::env;
use tico::tico;
use git2::Repository;

fn main() {
    print!("{}", cwd());
    match vcs() {
        Some(br) => println!(" {}", br),
        None => println!()
    }
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

fn vcs() -> Option<String> {
    let current_dir = env::var("PWD").unwrap();

    let repo = match Repository::open(current_dir) {
        Ok(r) => r,
        Err(_) => return None
    };

    let reference = repo.head().unwrap();

    if reference.is_branch() {
        Some(format!("{}", reference.shorthand().unwrap()))
    } else {
        let commit = reference.peel_to_commit().unwrap();
        let id = commit.id();
        Some(format!("{}", id))
    }
}
