use super::models::{GithubIssue, GithubPageInfo, GithubProjectV2, GithubResponse};
use reqwest::Client;
use serde::Serialize;
use std::error::Error;

use super::graphql::{query_issues::QUERY_ISSUES, query_project::QUERY_PROJECT};

/// GitHub client for interacting with GitHub's GraphQL API
#[derive(Debug, Clone)]
pub struct GitHubRepository {
    client: Client,
    token: String,
    api_url: String,
    organization: String,
}

#[derive(Debug, Serialize)]
struct GraphQLRequest {
    query: String,
    variables: serde_json::Value,
}

impl GitHubRepository {
    /// Create a new GitHub client
    pub fn new(token: String, api_url: String, organization: String) -> Self {
        let client = Client::new();
        let api_url = api_url;

        Self {
            client,
            token,
            api_url,
            organization,
        }
    }

    /// Get project information
    pub async fn get_project(
        &self,
        project_number: i32,
    ) -> Result<GithubProjectV2, Box<dyn Error>> {
        let variables = serde_json::json!({
            "organization": self.organization,
            "projectNumber": project_number
        });

        let response = self.execute_query(QUERY_PROJECT, variables).await?;
        let project_v2 = response.data.organization.project_v2;

        Ok(project_v2)
    }

    /// Get issues for a project with pagination
    pub async fn get_issues(
        &self,
        project_number: i32,
        limit: Option<i32>,
        after: Option<String>,
    ) -> Result<(Vec<GithubIssue>, GithubPageInfo), Box<dyn Error>> {
        println!("project_number: {}", project_number);
        let variables = serde_json::json!({
            "organization": self.organization,
            "projectNumber": project_number,
            "limit": limit.unwrap_or(100),
            "after": after
        });

        let response = match self.execute_query(QUERY_ISSUES, variables).await {
            Ok(response) => response,
            Err(e) => {
                println!("Error graphql github: {:?}", e);
                return Err(e);
            }
        };

        let issues = response.get_issues();
        let page_info = response.get_page_info();
        println!("Número de issues obtenidos {}", issues.len());

        Ok((issues, page_info))
    }

    /// Execute a GraphQL query against the GitHub API
    async fn execute_query(
        &self,
        query: &str,
        variables: serde_json::Value,
    ) -> Result<GithubResponse, Box<dyn Error>> {
        let request = GraphQLRequest {
            query: query.to_string(),
            variables,
        };

        let response = self
            .client
            .post(&self.api_url)
            .bearer_auth(&self.token)
            .header("User-Agent", "github-issues-migrator")
            .header("Accept", "application/json; charset=utf-8")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("GitHub API error: {}", response.status()).into());
        }

        let response_body: serde_json::Value = response.json().await?;

        if let Some(errors) = response_body.get("errors") {
            return Err(format!("GraphQL error: {}", errors).into());
        }

        let repsonse: GithubResponse = serde_json::from_value(response_body)?;

        Ok(repsonse)
    }
}
