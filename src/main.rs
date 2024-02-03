mod client;

use std::{env, process};

fn main() {
    let token = env::var("GITHUB_TOKEN").unwrap_or_else(|_e| {
        eprintln!("GITHUB_TOKEN environment variable is not set");
        process::exit(1);
    });

    let client = client::Client::new(&token);

    let mut comments = client.get_comments();

    println!("number of comments: {}", comments.len());

    for comment in comments.iter_mut() {
        println!("body = {}", comment.body);

        let issue = client::Issue {
            title: String::from("Created by comments2issues"),
            body: comment.body.clone(),
        };

        client.create_issue(issue);
    }
}
