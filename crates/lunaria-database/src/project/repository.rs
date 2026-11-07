use std::{path::PathBuf, sync::Arc};

use super::{database::ProjectDatabase, recent_project};
use crate::manager::DatabaseManager;
use crate::utils::unix_timestamp_secs;
use async_trait::async_trait;
use lunaria_core::project::{Project, ProjectError, ProjectRepository};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder, Set,
};

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
        created_at: i64,
        last_opend_at: i64,
    ) -> Result<recent_project::Model, sea_orm::DbErr> {
        let database = self.databases.app_database();

        if let Some(existing) = recent_project::Entity::find()
            .filter(recent_project::Column::ProjectPath.eq(path))
            .one(database)
            .await?
        {
            // update model
            let mut active = existing.into_active_model();
            active.project_name = Set(name.to_owned());
            active.created_at = Set(created_at);
            active.last_opened_at = Set(last_opend_at);
            active.update(database).await
        } else {
            // create model
            recent_project::ActiveModel {
                project_name: Set(name.to_owned()),
                project_path: Set(path.to_owned()),
                created_at: Set(created_at),
                last_opened_at: Set(last_opend_at),
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
            .save_recent_project(&name, &project_path, timestamp, timestamp)
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

    async fn open(&self, path: PathBuf) -> Result<Project, ProjectError> {
        // 1. 生成 project database
        let project_database = ProjectDatabase::open(&path)
            .await
            .map_err(ProjectError::storage)?;

        // 2. 获取 project 元信息
        let metadata = project_database
            .fetch_metadata()
            .await
            .map_err(ProjectError::storage)?;

        let project_path = project_database.directory().to_str().ok_or_else(|| {
            ProjectError::storage("project path is not valid UTF-8 and cannot be stored in SQLite")
        })?;

        let timestamp = unix_timestamp_secs().map_err(ProjectError::storage)?;

        let record = self
            .save_recent_project(&metadata.name, project_path, metadata.created_at, timestamp)
            .await
            .map_err(ProjectError::storage)?;

        self.databases.set_current_project(project_database).await;

        Ok(Project::new(
            record.id,
            record.project_name,
            PathBuf::from(record.project_path),
            record.created_at,
            record.last_opened_at,
        ))
    }

    async fn list_recent(&self) -> Result<Vec<Project>, ProjectError> {
        let records = recent_project::Entity::find()
            .order_by_desc(recent_project::Column::LastOpenedAt)
            .order_by_desc(recent_project::Column::Id)
            .all(self.databases.app_database())
            .await
            .map_err(ProjectError::storage)?;

        Ok(records
            .into_iter()
            .map(|record| {
                Project::new(
                    record.id,
                    record.project_name,
                    PathBuf::from(record.project_path),
                    record.created_at,
                    record.last_opened_at,
                )
            })
            .collect())
    }
}
