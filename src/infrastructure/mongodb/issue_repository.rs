use super::issue_model::IssueModel;
use crate::infrastructure::github::github_repository::GitHubRepository;
use mongodb::{
    bson::oid::ObjectId, error::Result as MongoResult, options::InsertManyOptions, Collection,
    Database,
};
use std::error::Error;
use tokio::time::{sleep, Duration};

pub struct IssueRepository {
    collection: Collection<IssueModel>,
    github_repository: GitHubRepository,
    snapshot_id: ObjectId,
}

impl IssueRepository {
    pub fn new(db: &Database, github_repository: GitHubRepository, snapshot_id: ObjectId) -> Self {
        let collection = db.collection::<IssueModel>("issues");
        Self {
            collection,
            github_repository,
            snapshot_id,
        }
    }

    pub async fn insert_many(&self, issues: Vec<IssueModel>) -> MongoResult<Vec<String>> {
        let options = InsertManyOptions::builder().ordered(false).build();
        let result = self
            .collection
            .insert_many(issues)
            .with_options(options)
            .await?;

        let ids = result
            .inserted_ids
            .values()
            .map(|id| id.as_object_id().unwrap().to_hex())
            .collect();
        Ok(ids)
    }

    pub async fn insert_from_github_project(
        &self,
        project_number: i32,
    ) -> Result<(), Box<dyn Error>> {
        let limit = 100;
        let mut after = "".to_string();

        loop {
            let (issues_page, page_info) = self
                .github_repository
                .get_issues(project_number, Some(limit), Some(after))
                .await?;
            // Check if issues_page is empty and break the loop
            if issues_page.is_empty() {
                println!("No more issues found for project #{}", project_number);
                break;
            }
            // Convert GitHub issues to IssueModel and batch insert them
            let issue_models: Vec<IssueModel> = issues_page
                .into_iter()
                .map(|issue| IssueModel::from_issue(issue, self.snapshot_id))
                .collect();

            if !issue_models.is_empty() {
                // Store the length before moving issue_models
                let issues_count = issue_models.len();
                // Insert the issue models in batch
                self.insert_many(issue_models).await?;
                println!(
                    "Inserted {} issues for project #{}",
                    issues_count, project_number
                );
            }

            if !page_info.has_next_page {
                break;
            }

            after = page_info.end_cursor.unwrap_or_default();
            sleep(Duration::from_secs(1)).await;
        }

        Ok(())
    }
}
