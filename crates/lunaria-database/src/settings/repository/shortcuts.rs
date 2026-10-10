use crate::DatabaseManager;
use async_trait::async_trait;
use lunaria_core::settings::error::SettingError;
use lunaria_core::settings::shortcuts::model::ShortcutOverrides;
use lunaria_core::settings::shortcuts::repository::ShortcutRepository;
use std::sync::Arc;

const SHORTCUTS_KEY: &str = "shortcuts";

pub struct Repository {
    databases: Arc<DatabaseManager>,
}

impl Repository {
    pub fn new(databases: Arc<DatabaseManager>) -> Arc<Self> {
        Arc::new(Self { databases })
    }
}

#[async_trait]
impl ShortcutRepository for Repository {
    async fn load(&self) -> Result<ShortcutOverrides, SettingError> {
        let overrides =
            super::load_setting::<ShortcutOverrides>(self.databases.app_database(), SHORTCUTS_KEY)
                .await?;

        Ok(overrides.unwrap_or_default())
    }

    async fn save(&self, overrides: &ShortcutOverrides) -> Result<(), SettingError> {
        let value = serde_json::to_string(overrides).map_err(SettingError::storage)?;

        super::save_setting(self.databases.app_database(), SHORTCUTS_KEY, value).await
    }
}
