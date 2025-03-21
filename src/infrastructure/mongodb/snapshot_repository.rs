use super::snapshot_model::SnapshotModel;
use anyhow::anyhow;

use mongodb::{bson::oid::ObjectId, Collection, Database};

pub struct SnapshotRepository {
    collection: Collection<SnapshotModel>,
}

impl SnapshotRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection::<SnapshotModel>("snapshots"),
        }
    }

    pub async fn create(
        &self,
        project_number: i32,
        project_id: ObjectId,
    ) -> Result<SnapshotModel, anyhow::Error> {
        let snapshot = SnapshotModel::new(project_number, project_id);
        let insert_result = self.collection.insert_one(snapshot.clone()).await;
        if insert_result.is_err() {
            let error_message = "Failed to create hotel";
            println!("{}", error_message);
            return Err(anyhow!(error_message));
        }

        let id = snapshot.id.to_string();
        println!("The snapshot was created successfully with id: {}", id);

        Ok(snapshot)
    }
}
