use crate::infrastructure::github::models::GithubProjectV2;
use bson::DateTime;
use mongodb::bson::oid::ObjectId; // Added import for ObjectId
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub github_id: String,
    pub owner_id: String,
    pub title: String,
    pub number: i32,
    pub url: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl From<GithubProjectV2> for ProjectModel {
    fn from(project: GithubProjectV2) -> Self {
        let now = DateTime::now();
        Self {
            id: ObjectId::new(),
            github_id: project.id,
            owner_id: "somossoftrek".to_string(),
            title: project.title,
            number: project.number,
            url: project.url,
            created_at: now,
            updated_at: now,
        }
    }
}
