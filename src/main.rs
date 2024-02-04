mod client;

use std::{env, process};

fn main() {
    let token = env::var("GITHUB_TOKEN").unwrap_or_else(|_e| {
        eprintln!("GITHUB_TOKEN environment variable is not set");
        process::exit(1);
    });

    let issue_url = "https://github.com/a2ikm/comments2issues/issues/2";
    let re = regex::Regex::new(
        r"https://github.com/(?<owner>[-a-zA-Z0-9]+)/(?<repo>[-_.a-zA-Z0-9]+)/issues/(?<issue_number>\d+)",
    )
    .unwrap();
    let Some(caps) = re.captures(issue_url) else {
        eprintln!("Given issue url is not correct");
        process::exit(1);
    };
    let Some(owner_match) = caps.name("owner") else {
        eprintln!("Given issue url doesn't contain owner");
        process::exit(1);
    };
    let Some(repo_match) = caps.name("repo") else {
        eprintln!("Given issue url doesn't contain repo");
        process::exit(1);
    };
    let Some(issue_number_match) = caps.name("issue_number") else {
        eprintln!("Given issue url doesn't contain issue_number");
        process::exit(1);
    };

    let client = client::Client::new(
        &token,
        owner_match.as_str(),
        repo_match.as_str(),
        issue_number_match.as_str(),
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
