use crate::DatabaseManager;
use crate::connection::create_table;
use crate::settings::app_setting;
use async_trait::async_trait;
use lunaria_core::settings::error::SettingError;
use lunaria_core::settings::shortcuts::model::ShortcutOverrides;
use lunaria_core::settings::shortcuts::repository::ShortcutRepository;
use sea_orm::sea_query::OnConflict;
use sea_orm::{EntityTrait, Set};
use std::sync::Arc;

const SHORTCUTS_KEY: &str = "shortcuts";

pub struct Repository {
    databases: Arc<DatabaseManager>,
}

impl Repository {
    pub async fn new(databases: Arc<DatabaseManager>) -> Arc<Self> {
        create_table(databases.app_database(), app_setting::Entity)
            .await
            .expect("Failed to initialize shortcut repository");
        Arc::new(Self { databases })
    }
}

#[async_trait]
impl ShortcutRepository for Repository {
    async fn load(&self) -> Result<ShortcutOverrides, SettingError> {
        let record = app_setting::Entity::find_by_id(SHORTCUTS_KEY)
            .one(self.databases.app_database())
            .await
            .map_err(SettingError::storage)?;

        match record {
            Some(record) => serde_json::from_str(&record.value).map_err(SettingError::storage),
            None => Ok(ShortcutOverrides::new()),
        }
    }

    async fn save(&self, overrides: &ShortcutOverrides) -> Result<(), SettingError> {
        let overrides_str = serde_json::to_string(overrides).map_err(SettingError::storage)?;

        app_setting::Entity::insert(app_setting::ActiveModel {
            key: Set(SHORTCUTS_KEY.to_owned()),
            value: Set(overrides_str),
        })
        .on_conflict(
            OnConflict::column(app_setting::Column::Key)
                .update_column(app_setting::Column::Value)
                .to_owned(),
        )
        .exec_without_returning(self.databases.app_database())
        .await
        .map_err(SettingError::storage)?;

        Ok(())
    }
}
