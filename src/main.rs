use git2::Repository;

fn main() {
    // 远程仓库 URL
    let repo_url = "https://github.com/shellingfordly/hello-rust.git";
    // 本地克隆仓库的路径
    let local_path = "/Users/apple/Projects/hello-rust/src/demo/";

    // 克隆远程仓库到本地
    match Repository::clone(repo_url, local_path) {
        Ok(_repo) => {
            println!("Repository cloned successfully");

            // 打开克隆的本地仓库
            if let Ok(_repo) = Repository::open(local_path) {
                // 在这里可以执行你想要的操作，比如修改、提交、推送等
                // 以下是一个简单的示例，输出当前分支和仓库 URL
                let head = _repo.head().unwrap();
                let branch_name = head.shorthand().unwrap_or("N/A");

                println!("Current branch: {}", branch_name);
                println!("Repository URL: {}", repo_url);
            } else {
                eprintln!("Failed to open the cloned repository");
            }
        }
        Err(err) => {
            eprintln!("Error cloning repository: {}", err);
        }
    }
}
