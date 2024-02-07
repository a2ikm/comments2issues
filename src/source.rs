pub struct SourceIssue {
    pub owner: String,
    pub repo: String,
    pub issue_number: String,
}

impl SourceIssue {
    pub fn parse(issue_url: &str) -> Result<SourceIssue, &'static str> {
        let re = regex::Regex::new(
          r"https://github.com/(?<owner>[-a-zA-Z0-9]+)/(?<repo>[-_.a-zA-Z0-9]+)/issues/(?<issue_number>\d+)",
        )
        .unwrap();

        let Some(caps) = re.captures(&issue_url) else {
            return Err("Given issue url is not correct");
        };
        let Some(owner_match) = caps.name("owner") else {
            return Err("Given issue url doesn't contain owner");
        };
        let Some(repo_match) = caps.name("repo") else {
            return Err("Given issue url doesn't contain repo");
        };
        let Some(issue_number_match) = caps.name("issue_number") else {
            return Err("Given issue url doesn't contain issue_number");
        };

        Ok(SourceIssue {
            owner: String::from(owner_match.as_str()),
            repo: String::from(repo_match.as_str()),
            issue_number: String::from(issue_number_match.as_str()),
        })
    }
}
