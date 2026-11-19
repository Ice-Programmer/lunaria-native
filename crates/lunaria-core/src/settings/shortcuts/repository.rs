use crate::settings::shortcuts::model::ShortcutOverrides;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ShortcutRepository: Send + Sync {
    async fn load(&self) -> Result<ShortcutOverrides>;

    async fn save(&self, overrides: &ShortcutOverrides) -> Result<()>;
}
