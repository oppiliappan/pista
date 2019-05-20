use std::env;
use std::path::Path;
use git2::{ Repository, Status };
use colored::*;

pub fn vcs_status() -> Option<(colored::ColoredString, colored::ColoredString)> {
    let current_dir = env::var("PWD").unwrap();

    let mut repo: Option<Repository> = None;
    let current_path = Path::new(&current_dir[..]);
    for path in current_path.ancestors() {
        match Repository::open(path) {
            Ok(r) => {
                repo = Some(r);
                break;
            }
            Err(_) => {},
        }
    }
    if repo.is_none() {
        return None
    }
    let repo = repo.unwrap();
    let reference = repo.head().unwrap();
    let mut branch;

    if reference.is_branch() {
        branch = format!("{}", reference.shorthand().unwrap()).bright_black();
    } else {
        let commit = reference.peel_to_commit().unwrap();
        let id = commit.id();
        branch = format!("{:.6}", id).bright_black();
    }

    let mut repo_stat = "".white();
    let git_clean_color          = env::var("GIT_CLEAN_COLOR").unwrap_or("green".into());
    let git_wt_modified_color    = env::var("GIT_WT_MODIFIED_COLOR").unwrap_or("red".into());
    let git_index_modified_color = env::var("GIT_INDEX_MODIFIED_COLOR").unwrap_or("yellow".into());

    let file_stats = repo.statuses(None).unwrap();
    for file in file_stats.iter() {
        match file.status() {
            // STATE: unstaged (working tree modified)
            Status::WT_NEW        | Status::WT_MODIFIED      |
            Status::WT_DELETED    | Status::WT_TYPECHANGE    |
            Status::WT_RENAMED => {
                let stat_char = env::var("GIT_WT_MODIFIED").unwrap_or("×".into());
                repo_stat = stat_char.color(&git_wt_modified_color[..]);
                break;
            },
            // STATE: staged (changes added to index)
            Status::INDEX_NEW     | Status::INDEX_MODIFIED   |
            Status::INDEX_DELETED | Status::INDEX_TYPECHANGE |
            Status::INDEX_RENAMED => {
                let stat_char = env::var("GIT_INDEX_MODIFIED").unwrap_or("±".into());
                repo_stat = stat_char.color(&git_index_modified_color[..]);
            },
            // STATE: comitted (changes have been saved in the repo)
            _ => {
                let stat_char = env::var("GIT_CLEAN").unwrap_or("·".into());
                repo_stat = stat_char.color(&git_clean_color[..]);
            }
        }
    }
    return Some((branch, repo_stat))
}
