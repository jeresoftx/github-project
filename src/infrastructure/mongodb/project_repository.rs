use crate::infrastructure::github::github_repository::GitHubRepository;
use anyhow::anyhow;
use mongodb::{bson::doc, Collection, Database};

use super::project_model::ProjectModel;

pub struct ProjectRepository {
    collection: Collection<ProjectModel>,
    github_repository: GitHubRepository,
}

impl ProjectRepository {
    pub fn new(db: &Database, github_repository: GitHubRepository) -> Self {
        Self {
            collection: db.collection::<ProjectModel>("projects"),
            github_repository,
        }
    }

    pub async fn get_by_number(&self, number: i32) -> Option<ProjectModel> {
        let filter = doc! { "number": number };
        let project = self.collection.find_one(filter).await.unwrap();
        project
    }

    pub async fn insert(&self, project: ProjectModel) -> Result<ProjectModel, anyhow::Error> {
        let insert_result = self.collection.insert_one(project.clone()).await;

        if insert_result.is_err() {
            let error_message = "Failed to create hotel";
            println!("{}", error_message);
            return Err(anyhow!(error_message));
        }
        let id = project.id.to_string();
        println!("The snapshot was created successfully with id: {}", id);
        Ok(project)
    }

    pub async fn find_or_create(
        &self,
        project_number: i32,
    ) -> Result<ProjectModel, Box<dyn std::error::Error>> {
        match self.get_by_number(project_number).await {
            Some(project) => Ok(project),
            None => {
                let github_project = self.github_repository.get_project(project_number).await?;
                let new_project = github_project.into();
                let result = self.insert(new_project).await?;
                Ok(result)
            }
        }
    }
}
