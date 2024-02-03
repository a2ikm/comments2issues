use std::{env, process};

#[derive(serde::Deserialize)]
struct Comment {
    body: String,
}

fn main() {
    let token = env::var("GITHUB_TOKEN").unwrap_or_else(|_e| {
        eprintln!("GITHUB_TOKEN environment variable is not set");
        process::exit(1);
    });

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Accept",
        reqwest::header::HeaderValue::from_str("application/vnd.github+json").unwrap(),
    );
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(format!("Bearer {token}", token = token).as_str())
            .unwrap(),
    );
    headers.insert(
        "X-GitHub-Api-Version",
        reqwest::header::HeaderValue::from_str("2022-11-28").unwrap(),
    );

    let client = reqwest::blocking::ClientBuilder::new()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ))
        .default_headers(headers)
        .build()
        .unwrap_or_else(|_e| {
            eprintln!("Failed to build client");
            process::exit(1);
        });

    let comments_url = reqwest::Url::parse(
        format!(
            "https://api.github.com/repos/{owner}/{repo}/issues/{issue_number}/comments",
            owner = "a2ikm",
            repo = "comments2issues",
            issue_number = "1"
        )
        .as_str(),
    )
    .unwrap();

    let response = client.get(comments_url).send().unwrap_or_else(|_e| {
        eprintln!("Failed to get the issue");
        process::exit(1);
    });

    let mut comments: Vec<Comment> = serde_json::from_str(response.text().unwrap().as_str())
        .unwrap_or_else(|_e| {
            eprintln!("Failed to parse JSON response");
            process::exit(1);
        });

    for comment in comments.iter_mut() {
        println!("{}", comment.body);
    }
}
