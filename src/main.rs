mod cwd;
mod prompt_char;
mod vcs;
mod venv;

fn main() {
    println!("{}", pista());
}

fn pista() -> String {
    let cwd = cwd::cwd();
    let (branch, status) = vcs::vcs_status().unwrap_or(("".into(), "".into()));
    let venv = venv::get_name();
    let prompt_char = prompt_char::get_char();
    format!("{cwd} {branch} {status}\n{venv}{pchar} ",
            cwd=cwd,
            branch=branch,
            status=status,
            venv=venv,
            pchar=prompt_char
            )
}

fn pista_minimal() -> String {
    let cwd = cwd::cwd();
    let (branch, status) = vcs::vcs_status().unwrap_or(("".into(), "".into()));
    let venv = venv::get_name();
    let prompt_char = prompt_char::get_char();
    format!("{cwd} {branch}{status}{venv}{pchar}",
            cwd=cwd,
            branch=branch,
            status=status,
            venv=venv,
            pchar=prompt_char
            )
}
