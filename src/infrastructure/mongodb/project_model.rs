use crate::infrastructure::github::models::GithubProjectV2;
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId; // Added import for ObjectId
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub github_id: String,
    pub title: String,
    pub number: i32,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<GithubProjectV2> for ProjectModel {
    fn from(project: GithubProjectV2) -> Self {
        Self {
            id: ObjectId::new(),
            github_id: project.id,
            title: project.title,
            number: project.number,
            url: project.url,
            created_at: DateTime::parse_from_rfc3339(&project.created_at)
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&project.updated_at)
                .unwrap()
                .with_timezone(&Utc),
        }
    }
}
