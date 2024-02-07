mod client;
mod source;

use clap::Parser;
use std::{env, process};

#[derive(clap::Parser, Debug)]
#[command()]
struct Args {
    /// Source issue URL
    issue_url: String,
}

fn main() {
    let token = env::var("GITHUB_TOKEN").unwrap_or_else(|_e| {
        eprintln!("GITHUB_TOKEN environment variable is not set");
        process::exit(1);
    });

    let args = Args::parse();
    let source_issue = source::SourceIssue::parse(&args.issue_url);

    let client = client::Client::new(
        &token,
        source_issue.owner,
        source_issue.repo,
        source_issue.issue_number,
    );

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
