use crate::settings::language::model::Language;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait LanguageRepository: Send + Sync {
    async fn load(&self) -> Result<Option<Language>>;

    async fn save(&self, language: Language) -> Result<()>;
}
