use std::{env, process};

pub struct Client {
    owner: String,
    repo: String,
    issue_number: String,
    client: reqwest::blocking::Client,
}

#[derive(serde::Deserialize)]
pub struct Comment {
    pub body: String,
}

#[derive(serde::Serialize)]
pub struct Issue {
    pub title: String,
    pub body: String,
}

impl Client {
    pub fn new(token: &str, owner: String, repo: String, issue_number: String) -> Client {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Accept",
            reqwest::header::HeaderValue::from_str("application/vnd.github+json").unwrap(),
        );
        headers.insert(
            "Authorization",
            reqwest::header::HeaderValue::from_str(
                format!("Bearer {token}", token = token).as_str(),
            )
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

        Client {
            owner,
            repo,
            issue_number,
            client,
        }
    }

    pub fn get_comments(&self) -> Vec<Comment> {
        let url = reqwest::Url::parse(&self.comments_api_url()).unwrap();
        let response = self.client.get(url).send().unwrap_or_else(|_e| {
            eprintln!("Failed to get the issue");
            process::exit(1);
        });

        let json = response.text().unwrap();
        let comments: Vec<Comment> = serde_json::from_str(&json).unwrap_or_else(|_e| {
            eprintln!("Failed to parse JSON response");
            process::exit(1);
        });

        comments
    }

    pub fn create_issue(&self, issue: Issue) {
        let url = reqwest::Url::parse(&self.issues_api_url()).unwrap();

        let response = self
            .client
            .post(url)
            .json(&issue)
            .send()
            .unwrap_or_else(|_e| {
                eprintln!("Failed to create a new issue");
                process::exit(1);
            });

        println!("{}", response.status().as_str());
    }

    fn comments_api_url(&self) -> String {
        format!(
            "https://api.github.com/repos/{owner}/{repo}/issues/{issue_number}/comments",
            owner = self.owner,
            repo = self.repo,
            issue_number = self.issue_number,
        )
    }

    fn issues_api_url(&self) -> String {
        format!(
            "https://api.github.com/repos/{owner}/{repo}/issues",
            owner = self.owner,
            repo = self.repo,
        )
    }
}
