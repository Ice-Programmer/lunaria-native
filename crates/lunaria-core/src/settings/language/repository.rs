use crate::settings::error::SettingError;
use crate::settings::language::model::Language;
use async_trait::async_trait;

#[async_trait]
pub trait LanguageRepository: Send + Sync {
    async fn load(&self) -> Result<Option<Language>, SettingError>;

    async fn save(&self, language: Language) -> Result<(), SettingError>;
}
