use crate::DatabaseManager;
use anyhow::Result;
use async_trait::async_trait;
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
    async fn load(&self) -> Result<ShortcutOverrides> {
        let overrides =
            super::load_setting::<ShortcutOverrides>(self.databases.app_database(), SHORTCUTS_KEY)
                .await?;

        Ok(overrides.unwrap_or_default())
    }

    async fn save(&self, overrides: &ShortcutOverrides) -> Result<()> {
        let value = serde_json::to_string(overrides)?;

        super::save_setting(self.databases.app_database(), SHORTCUTS_KEY, value).await
    }
}
