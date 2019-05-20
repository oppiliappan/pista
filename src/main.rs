mod cwd;
mod prompt_char;
mod vcs;
mod venv;

use colored::*;

fn main() {
    print!("{}", cwd::cwd());
    let (branch, status) = vcs::vcs_status().unwrap_or(("".into(), "".into()));
    println!(" {} {}", branch, status.dimmed());
    print!("{}{} ", venv::get_name(), prompt_char::prompt_char());
}
