use crate::infrastructure::github::models::{GithubIssue, GithubUser};
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId; // Added import for ObjectId
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueModel {
    #[serde(rename = "_id")]
    id: ObjectId,
    snapshot_id: ObjectId,
    github_issue_id: String,
    url: String,
    title: String,
    state: String,
    state_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    estimate: Option<f64>,
    hours: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iteration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assigned: Option<UserModel>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    closed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserModel {
    id: String,
    login: String,
    url: String,
}

impl From<GithubUser> for UserModel {
    fn from(user: GithubUser) -> Self {
        UserModel {
            id: user.id,
            login: user.login,
            url: user.url,
        }
    }
}
// ...existing code...
impl IssueModel {
    pub fn from_issue(issue: GithubIssue, snapshot_id: ObjectId) -> Self {
        // Get the first label if available
        let label = if !issue.labels.nodes.is_empty() {
            Some(issue.labels.nodes[0].name.clone())
        } else {
            None
        };

        // Get the first assignee if available
        let assigned = if !issue.assignees.nodes.is_empty() {
            Some(issue.assignees.nodes[0].clone().into())
        } else {
            None
        };

        // Default values for optional fields
        let mut estimate: Option<f64> = None;
        let mut iteration: Option<String> = None;
        let mut start_date: Option<String> = None;
        let mut end_date: Option<String> = None;
        let mut hours: Option<f64> = None;

        // Extract custom field values from project_items
        for node in issue.project_items.nodes {
            for field_value in node.field_values.nodes {
                if let Some(field) = field_value.field {
                    if field.name.is_empty() {
                        continue;
                    }
                    match field.name.as_str() {
                        "Estimate" => estimate = field_value.number,
                        "Iteration" => iteration = field_value.text,
                        "Start Date" => start_date = field_value.text,
                        "End Date" => end_date = field_value.text,
                        "Hours" => hours = field_value.number,
                        _ => {}
                    }
                }
            }
        }

        IssueModel {
            id: ObjectId::new(), // Generate a new ObjectId
            snapshot_id,         // Use the provided snapshot_id
            github_issue_id: issue.id,
            url: issue.url,
            title: issue.title,
            state: issue.state,
            state_reason: issue.state_reason,
            label,
            estimate,
            iteration,
            start_date,
            end_date,
            hours,
            assigned,
            created_at: DateTime::parse_from_rfc3339(&issue.created_at)
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&issue.updated_at)
                .unwrap()
                .with_timezone(&Utc),
            closed_at: issue.closed_at.as_ref().map(|date| {
                DateTime::parse_from_rfc3339(date)
                    .unwrap()
                    .with_timezone(&Utc)
            }),
        }
    }
}
