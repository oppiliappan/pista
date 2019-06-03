mod cwd;
mod prompt_char;
mod vcs;
mod venv;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Pista")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("minimal")
             .short("m")
             .long("minimal")
             .help("use minimal variant")
        )
        .get_matches();
    if matches.is_present("minimal") {
        println!("{}", pista_minimal());
    } else {
        println!("{}", pista());
    }
}

fn pista() -> String {
    let cwd = cwd::cwd();
    let (branch, status) = vcs::vcs_status().unwrap_or(("".into(), "".into()));
    let venv = venv::get_name();
    let prompt_char = prompt_char::get_char();
    format!("%{{{cwd} {branch} {status}%}} %{{\n{venv}{pchar}%}} ",
            cwd=cwd,
            branch=branch,
            status=status,
            venv=venv,
            pchar=prompt_char
            )
}

fn pista_minimal() -> String {
    let cwd = cwd::cwd();
    let vcs_tuple = vcs::vcs_status();
    let mut vcs_component = String::new();
    if let Some((branch, status)) = vcs_tuple {
        vcs_component = format!(" [{} {}] ", branch, status);
    } else {
        vcs_component.push(' ');
    }
    let venv = venv::get_name();
    let prompt_char = prompt_char::get_char();
    format!("{cwd}{vcs}{venv}{pchar} ",
            cwd=cwd,
            vcs=vcs_component,
            venv=venv,
            pchar=prompt_char
            )
}
