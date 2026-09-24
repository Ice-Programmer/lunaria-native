use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use tokio::sync::RwLock;

use crate::DatabaseError;
use crate::database::app_database::AppDatabase;
use crate::database::project_database::ProjectDatabase;

pub struct DatabaseManager {
    app_database: AppDatabase,
    current_project: RwLock<Option<Arc<ProjectDatabase>>>,
}

impl DatabaseManager {
    pub async fn initialize(app_data_directory: impl AsRef<Path>) -> Result<Self, DatabaseError> {
        let app_database = AppDatabase::open_or_create(app_data_directory.as_ref()).await?;

        Ok(Self {
            app_database,
            current_project: RwLock::new(None),
        })
    }

    pub async fn current_project_directory(&self) -> Option<PathBuf> {
        self.current_project()
            .await
            .map(|project| project.directory().to_path_buf())
    }

    pub(crate) fn app_database(&self) -> &sea_orm::DatabaseConnection {
        self.app_database.connection()
    }

    pub(crate) async fn set_current_project(&self, project: ProjectDatabase) {
        *self.current_project.write().await = Some(Arc::new(project));
    }

    pub(crate) async fn current_project(&self) -> Option<Arc<ProjectDatabase>> {
        self.current_project.read().await.clone()
    }
}
