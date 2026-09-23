use crate::project::error::ProjectError;
use crate::project::model::Project;
use async_trait::async_trait;
use std::path::PathBuf;

#[async_trait]
pub trait ProjectRepository: Send + Sync {
    async fn create(&self, name: String, path: PathBuf) -> Result<Project, ProjectError>;
}
