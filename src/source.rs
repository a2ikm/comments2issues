use std::process;

pub struct SourceIssue {
    pub owner: String,
    pub repo: String,
    pub issue_number: String,
}

impl SourceIssue {
    pub fn parse(issue_url: &str) -> SourceIssue {
        let re = regex::Regex::new(
          r"https://github.com/(?<owner>[-a-zA-Z0-9]+)/(?<repo>[-_.a-zA-Z0-9]+)/issues/(?<issue_number>\d+)",
        )
        .unwrap();
        let Some(caps) = re.captures(&issue_url) else {
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

        SourceIssue {
            owner: String::from(owner_match.as_str()),
            repo: String::from(repo_match.as_str()),
            issue_number: String::from(issue_number_match.as_str()),
        }
    }
}
