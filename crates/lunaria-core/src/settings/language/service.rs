use crate::settings::error::SettingError;
use crate::settings::language::model::Language;
use crate::settings::language::repository::LanguageRepository;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct LanguageService {
    language: RwLock<Language>,
    repository: Arc<dyn LanguageRepository>,
}

impl LanguageService {
    pub async fn new(repository: Arc<dyn LanguageRepository>) -> Arc<Self> {
        let language = match repository.load().await {
            Ok(language) => language.unwrap_or_default(),
            Err(error) => {
                eprintln!("Failed to load language, using default: {error}");

                Language::default()
            }
        };

        Arc::new(Self {
            language: RwLock::new(language),
            repository,
        })
    }

    pub async fn current(&self) -> Language {
        *self.language.read().await
    }

    pub async fn set(&self, language: Language) -> Result<(), SettingError> {
        let mut current = self.language.write().await;

        self.repository.save(language).await?;
        *current = language;

        Ok(())
    }
}
