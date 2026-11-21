use crate::settings::language::model::Language;
use crate::settings::language::repository::LanguageRepository;
use anyhow::Result;
use lunaria_di::service;
use std::sync::Arc;

#[service]
pub struct LanguageService {
    repository: Arc<dyn LanguageRepository>,
}

impl LanguageService {
    pub fn new(repository: Arc<dyn LanguageRepository>) -> Self {
        Self { repository }
    }

    pub async fn current(&self) -> Language {
        match self.repository.load().await {
            Ok(language) => language.unwrap_or_default(),
            Err(error) => {
                eprintln!("Failed to load language, using default, {error:#}");
                Language::default()
            }
        }
    }

    pub async fn set(&self, language: Language) -> Result<()> {
        self.repository.save(language).await
    }
}
