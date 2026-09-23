use crate::project::error::ProjectError;
use crate::project::model::Project;
use crate::project::repository::ProjectRepository;
use crate::project::validator::ProjectValidator;
use std::path::PathBuf;
use std::sync::Arc;

pub struct ProjectService {
    repository: Arc<dyn ProjectRepository>,
}

impl ProjectService {
    pub fn new(repository: Arc<dyn ProjectRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(&self, name: String, path: PathBuf) -> Result<Project, ProjectError> {
        let name = name.trim().to_owned();

        ProjectValidator::validate_name(&name)?;
        ProjectValidator::validate_path(&path)?;

        self.repository.create(name, path).await
    }
}
