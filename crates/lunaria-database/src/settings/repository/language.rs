use crate::DatabaseManager;
use async_trait::async_trait;
use lunaria_core::settings::error::SettingError;
use lunaria_core::settings::language::model::Language;
use lunaria_core::settings::language::repository::LanguageRepository;
use std::sync::Arc;

const LANGUAGE_KEY: &str = "language";

pub struct Repository {
    databases: Arc<DatabaseManager>,
}

impl Repository {
    pub fn new(databases: Arc<DatabaseManager>) -> Arc<Self> {
        Arc::new(Self { databases })
    }
}

#[async_trait]
impl LanguageRepository for Repository {
    async fn load(&self) -> Result<Option<Language>, SettingError> {
        super::load_setting(self.databases.app_database(), LANGUAGE_KEY).await
    }

    async fn save(&self, language: Language) -> Result<(), SettingError> {
        let value = serde_json::to_string(&language).map_err(SettingError::storage)?;

        super::save_setting(self.databases.app_database(), LANGUAGE_KEY, value).await
    }
}
