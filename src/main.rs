use std::{env, process};

fn main() {
    let token = env::var("GITHUB_TOKEN").unwrap_or_else(|_| {
        eprintln!("GITHUB_TOKEN environment variable is not set");
        process::exit(1);
    });

    println!("token = {token}", token = token);
}
