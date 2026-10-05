use crate::error::SettingError;
use crate::shortcuts::model::ShortcutOverrides;
use async_trait::async_trait;

#[async_trait]
pub trait ShortcutRepository: Send + Sync {
    async fn load(&self) -> Result<ShortcutOverrides, SettingError>;

    async fn save(&self, overrides: &ShortcutOverrides) -> Result<(), SettingError>;
}
