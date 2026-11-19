use std::{path::PathBuf, sync::Arc};

use super::{database::ProjectDatabase, recent_project};
use crate::manager::DatabaseManager;
use crate::utils::unix_timestamp_secs;
use anyhow::{Context, Result, bail};
use async_trait::async_trait;
use lunaria_core::project::{Project, ProjectRepository};
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
    ) -> Result<recent_project::Model> {
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
            Ok(active.update(database).await?)
        } else {
            // create model
            Ok(recent_project::ActiveModel {
                project_name: Set(name.to_owned()),
                project_path: Set(path.to_owned()),
                created_at: Set(created_at),
                last_opened_at: Set(last_opend_at),
                ..Default::default()
            }
            .insert(database)
            .await?)
        }
    }
}

#[async_trait]
impl ProjectRepository for Repository {
    async fn create(&self, name: String, parent_directory: PathBuf) -> Result<Project> {
        let timestamp = unix_timestamp_secs()?;
        let project_database = ProjectDatabase::create(&parent_directory, &name, timestamp).await?;

        let project_path = match project_database.directory().to_str() {
            Some(path) => path.to_owned(),
            None => {
                project_database.rollback().await;
                bail!("project path is not valid UTF-8 and cannot be stored in SQLite");
            }
        };

        let record = match self
            .save_recent_project(&name, &project_path, timestamp, timestamp)
            .await
        {
            Ok(record) => record,
            Err(error) => {
                project_database.rollback().await;
                return Err(error);
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

    async fn open(&self, path: PathBuf) -> Result<Project> {
        // 1. 生成 project database
        let project_database = ProjectDatabase::open(&path).await?;

        // 2. 获取 project 元信息
        let metadata = project_database.fetch_metadata().await?;

        let project_path = project_database
            .directory()
            .to_str()
            .context("project path is not valid UTF-8 and cannot be stored in SQLite")?;

        let timestamp = unix_timestamp_secs()?;

        let record = self
            .save_recent_project(&metadata.name, project_path, metadata.created_at, timestamp)
            .await?;

        self.databases.set_current_project(project_database).await;

        Ok(Project::new(
            record.id,
            record.project_name,
            PathBuf::from(record.project_path),
            record.created_at,
            record.last_opened_at,
        ))
    }

    async fn list_recent(&self) -> Result<Vec<Project>> {
        let records = recent_project::Entity::find()
            .order_by_desc(recent_project::Column::LastOpenedAt)
            .order_by_desc(recent_project::Column::Id)
            .all(self.databases.app_database())
            .await?;

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
