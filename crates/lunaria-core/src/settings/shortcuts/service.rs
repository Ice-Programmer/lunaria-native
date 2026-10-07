use crate::settings::error::SettingError;
use crate::settings::shortcuts::catalog;
use crate::settings::shortcuts::model::{
    Shortcut, ShortcutAction, ShortcutDefinition, ShortcutItem, ShortcutOverrides,
};
use crate::settings::shortcuts::repository::ShortcutRepository;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ShortcutService {
    definitions: Vec<ShortcutDefinition>,
    overrides: RwLock<ShortcutOverrides>,
    repository: Arc<dyn ShortcutRepository>,
}

impl ShortcutService {
    pub async fn new(repository: Arc<dyn ShortcutRepository>) -> Arc<Self> {
        let overrides = repository
            .load()
            .await
            .expect("Failed to load shortcut overrides");

        Arc::new(Self {
            definitions: catalog::default_shortcuts(),
            overrides: RwLock::new(overrides),
            repository,
        })
    }

    pub async fn list(&self) -> Result<Vec<ShortcutItem>, SettingError> {
        let overrides = self.overrides.read().await;

        let items = self
            .definitions
            .iter()
            .map(|definition| ShortcutItem {
                definition: definition.clone(),
                custom: overrides
                    .get(&definition.action)
                    .cloned()
                    .unwrap_or_else(|| Some(definition.defaults.clone())),
            })
            .collect();

        Ok(items)
    }

    pub async fn get(&self, action: ShortcutAction) -> Option<Vec<Shortcut>> {
        let overrides = self.overrides.read().await;

        overrides.get(&action).cloned().unwrap_or_else(|| {
            self.definitions
                .iter()
                .find(|definition| definition.action == action)
                .map(|definition| definition.defaults.clone())
        })
    }

    pub async fn set(
        &self,
        action: ShortcutAction,
        shortcuts: Vec<Shortcut>,
    ) -> Result<(), SettingError> {
        let is_default = self
            .definitions
            .iter()
            .any(|definition| definition.action == action && definition.defaults == shortcuts);

        self.update(move |overrides| {
            if shortcuts.is_empty() {
                overrides.insert(action, None);
            } else if is_default {
                overrides.remove(&action);
            } else {
                overrides.insert(action, Some(shortcuts));
            }
        })
        .await
    }

    pub async fn remove_shortcut(
        &self,
        action: ShortcutAction,
        shortcut: &Shortcut,
    ) -> Result<(), SettingError> {
        let defaults = self
            .definitions
            .iter()
            .find(|definition| definition.action == action)
            .map(|definition| definition.defaults.clone());

        self.update(move |overrides| {
            // 获取当前生效的 shortcuts
            let mut shortcuts = match overrides.get(&action) {
                Some(Some(shortcuts)) => shortcuts.clone(),
                Some(None) => return,
                None => match &defaults {
                    Some(shortcuts) => shortcuts.clone(),
                    None => return,
                },
            };

            let previous_len = shortcuts.len();
            shortcuts.retain(|item| item != shortcut);
            if shortcuts.len() == previous_len {
                return;
            }

            if shortcuts.is_empty() {
                overrides.insert(action, None);
            } else if defaults.as_ref() == Some(&shortcuts) {
                overrides.remove(&action);
            } else {
                overrides.insert(action, Some(shortcuts));
            }
        })
        .await
    }

    pub async fn disable(&self, action: ShortcutAction) -> Result<(), SettingError> {
        self.update(move |overrides| {
            overrides.insert(action, None);
        })
        .await
    }

    pub async fn reset(&self, action: ShortcutAction) -> Result<(), SettingError> {
        self.update(move |overrides| {
            overrides.remove(&action);
        })
        .await
    }

    async fn update(
        &self,
        change: impl FnOnce(&mut ShortcutOverrides),
    ) -> Result<(), SettingError> {
        let mut current = self.overrides.write().await;
        let mut next = current.clone();

        change(&mut next);
        self.repository.save(&next).await?;

        *current = next;
        Ok(())
    }
}
