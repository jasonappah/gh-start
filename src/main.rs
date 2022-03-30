use clap::Parser;
use inquire::{Select, Text};
use std::env;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Name for new subdirectory. Defaults to original repository name.
    #[clap(short, long)]
    name: Option<String>,

    /// Repo to clone. Can be a Git URL or org/repo
    #[clap(short, long)]
    repo: Option<String>,

    /// Path to clone to. Defaults to current directory.
    #[clap(short, long)]
    path: Option<String>,
}

// TODO: less sus error handling
fn main() {
    let cli = Args::parse();
    let repo = cli.repo.unwrap_or_else(|| {
        Select::new("Choose template:", get_repos())
            .with_help_message("If want more repos here, star them on GitHub!")
            .prompt()
            .unwrap()
    });
    let name = cli.name.unwrap_or_else(|| {
        Text::new("What should the repo be called?")
            .with_initial_value(&repo)
            .prompt()
            .unwrap()
    });

    let path = cli
        .path
        .unwrap_or_else(|| env::current_dir().unwrap().to_str().unwrap().to_owned());

    clone_repo(&repo, &name, &path);
}

fn clone_repo(repo: &str, name: &str, path: &str) {
    // TODO: Make this work lol
    // gh repo clone <repo> <dir> -- --depth=1
    // rm .git
    // git init
    // gh repo create
    println!("Done cloning {} to {} in {}!", repo, name, path);
}

fn get_repos() -> Vec<String> {
    // TODO: pull all starred repos marked as templates and display them as a list

    // let res = reqwest::get("http://httpbin.org/get").await?;
    // println!("Status: {}", res.status());
    // println!("Headers:\n{:#?}", res.headers());

    // let body = res.json().await?;
    // println!("Body:\n{}", body);
    // return repos
    vec![
        "sarthaktexas/perfect".to_string(),
        "arashnrim/whats-next".to_string(),
    ]
}
