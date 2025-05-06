use bson::DateTime;
use mongodb::bson::oid::ObjectId; // Added import for ObjectId
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub project_number: i32,
    pub project_id: ObjectId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<DateTime>,
}
impl SnapshotModel {
    pub fn new(project_number: i32, project_id: ObjectId) -> Self {
        let now = DateTime::now();
        Self {
            id: ObjectId::new(),
            project_id,
            project_number,
            created_at: Some(now),
            updated_at: Some(now),
            closed_at: None,
        }
    }
}
