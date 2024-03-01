use std::env;
use std::fs;
mod git;
use git::{git_clone, git_commit, git_pull, git_push};

pub fn get_path() -> &'static str {
    let current_dir = match env::current_dir() {
        Ok(path) => path.join("templates/").to_string_lossy().into_owned(),
        Err(err) => {
            eprintln!("Failed to get current directory: {}", err);
            String::new() // Return an empty string or handle the error differently
        }
    };

    Box::leak(current_dir.into_boxed_str())
}

fn main() {
    let repo_url: &str = "https://github.com/shellingfordly/hello-rust.git";
    let local_path: &str = get_path();

    let mut is_exist = false;

    match fs::metadata(local_path) {
        Ok(metadata) => is_exist = metadata.is_dir(),
        Err(_) => {
            println!("Folder does not exist at path: {}", local_path);
        }
    }

    if !is_exist {
        let clone_status = git_clone(repo_url, local_path);

        if !clone_status {
            println!("clone is failed.");
            return;
        }
    }

    println!("test commit");

    git_pull(repo_url, local_path);

    git_commit(local_path);

    git_push(local_path);
}
