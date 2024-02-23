use git2::{FetchOptions, RemoteCallbacks, Repository};
use std::fs;

fn git_clone(repo_url: &str, local_path: &str) -> bool {
    match Repository::clone(repo_url, local_path) {
        Ok(_repo) => {
            println!("Repository cloned successfully");
            return true;
        }
        Err(err) => {
            eprintln!("Error cloning repository: {}", err);
            return false;
        }
    }
}

fn git_pull(local_path: &str) {
    let repo = match Repository::open(local_path) {
        Ok(repo) => repo,
        Err(_) => {
            // 远程仓库 URL
            let remote_url = "https://github.com/your-username/your-repo.git";

            // 创建新的仓库并从远程克隆
            git2::Repository::clone(remote_url, local_path).expect("Failed to clone from remote")
        }
    };
    // 获取远程仓库
    let remote_name = "origin"; // 远程仓库的名称
    let mut remote: git2::Remote = repo
        .find_remote(remote_name)
        .expect("Failed to find remote");

    // 设置回调函数（例如，用于认证）
    let mut callbacks = RemoteCallbacks::new();
    // 这里可以添加认证信息等回调函数

    // 执行拉取操作
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    remote
        .fetch::<&str>(&[], Some(&mut fetch_options), None)
        .expect("Failed to fetch from remote");
}

fn main() {
    let repo_url = "https://github.com/shellingfordly/hello-rust.git";
    let local_path = "/Users/apple/Projects/hello-rust/templates/";

    let mut is_exist = false;

    match fs::metadata(local_path) {
        Ok(metadata) => is_exist = metadata.is_dir(),
        Err(_) => {
            println!("Folder does not exist at path: {}", local_path);
        }
    }
    println!("is_exist: {}", is_exist);

    if !is_exist {
        let clone_status = git_clone(repo_url, local_path);

        if !clone_status {
            println!("clone is failed.");
            return;
        }
    }

    git_pull(local_path);
}
