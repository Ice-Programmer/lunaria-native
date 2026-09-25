use std::{path::PathBuf, sync::Arc};

use async_trait::async_trait;
use lunaria_core::project::{Project, ProjectError, ProjectRepository};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set};

use super::{database::ProjectDatabase, recent_project};
use crate::manager::DatabaseManager;
use crate::utils::unix_timestamp_secs;

pub struct Repository {
    databases: Arc<DatabaseManager>,
}

impl Repository {
    pub fn new(databases: Arc<DatabaseManager>) -> Self {
        Self { databases }
    }

    async fn save_recent_project(
        &self,
        name: &str,
        path: &str,
        timestamp: i64,
    ) -> Result<recent_project::Model, sea_orm::DbErr> {
        let database = self.databases.app_database();

        if let Some(existing) = recent_project::Entity::find()
            .filter(recent_project::Column::ProjectPath.eq(path))
            .one(database)
            .await?
        {
            let mut active = existing.into_active_model();
            active.project_name = Set(name.to_owned());
            active.created_at = Set(timestamp);
            active.last_opened_at = Set(timestamp);
            active.update(database).await
        } else {
            recent_project::ActiveModel {
                project_name: Set(name.to_owned()),
                project_path: Set(path.to_owned()),
                created_at: Set(timestamp),
                last_opened_at: Set(timestamp),
                ..Default::default()
            }
            .insert(database)
            .await
        }
    }
}

#[async_trait]
impl ProjectRepository for Repository {
    async fn create(
        &self,
        name: String,
        parent_directory: PathBuf,
    ) -> Result<Project, ProjectError> {
        let timestamp = unix_timestamp_secs().map_err(ProjectError::storage)?;
        let project_database = ProjectDatabase::create(&parent_directory, &name, timestamp)
            .await
            .map_err(ProjectError::storage)?;

        let project_path = match project_database.directory().to_str() {
            Some(path) => path.to_owned(),
            None => {
                project_database.rollback().await;
                return Err(ProjectError::storage(
                    "project path is not valid UTF-8 and cannot be stored in SQLite",
                ));
            }
        };

        let record = match self
            .save_recent_project(&name, &project_path, timestamp)
            .await
        {
            Ok(record) => record,
            Err(error) => {
                project_database.rollback().await;
                return Err(ProjectError::storage(error));
            }
        };

        self.databases.set_current_project(project_database).await;

        Ok(Project::new(
            record.id,
            record.project_name,
            PathBuf::from(record.project_path),
            record.created_at,
            record.last_opened_at,
        ))
    }
}
