use crate::DatabaseManager;
use anyhow::Result;
use async_trait::async_trait;
use lunaria_core::settings::language::model::Language;
use lunaria_core::settings::language::repository::LanguageRepository;
use lunaria_di::{SourceKind, repository};
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
#[repository(key = SourceKind::Local)]
impl LanguageRepository for Repository {
    async fn load(&self) -> Result<Option<Language>> {
        super::load_setting(self.databases.app_database(), LANGUAGE_KEY).await
    }

    async fn save(&self, language: Language) -> Result<()> {
        let value = serde_json::to_string(&language)?;

        super::save_setting(self.databases.app_database(), LANGUAGE_KEY, value).await
    }
}
