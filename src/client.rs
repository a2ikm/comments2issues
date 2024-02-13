use std::env;

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
pub struct Issue<'a> {
    pub title: String,
    pub body: String,
    pub labels: Option<Vec<&'a str>>,
}

impl Client {
    pub fn new(
        token: String,
        owner: String,
        repo: String,
        issue_number: String,
    ) -> Result<Client, &'static str> {
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

        let Ok(client) = reqwest::blocking::ClientBuilder::new()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .default_headers(headers)
            .build()
        else {
            return Err("Failed to build client".into());
        };

        Ok(Client {
            owner,
            repo,
            issue_number,
            client,
        })
    }

    pub fn get_comments(&self) -> Result<Vec<Comment>, &'static str> {
        let url = reqwest::Url::parse(&self.comments_api_url()).unwrap();
        let Ok(response) = self.client.get(url).send() else {
            return Err("Failed to get the issue");
        };

        let json = response.text().unwrap();
        let Ok(comments): Result<Vec<Comment>, serde_json::Error> = serde_json::from_str(&json)
        else {
            return Err("Failed to parse JSON response");
        };

        Ok(comments)
    }

    pub fn create_issue(
        &self,
        issue: Issue,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let url = reqwest::Url::parse(&self.issues_api_url()).unwrap();

        self.client.post(url).json(&issue).send()
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
