use crate::settings::shortcuts::model::ShortcutAction;
use gpui_kit::Modifiers;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SettingError {
    #[error("unsupported platform: {0}")]
    UnsupportedPlatform(String),

    #[error("invalid key: {0}")]
    InvalidKey(String),

    #[error("shortcut storage operation failed: {0}")]
    Storage(String),
}

impl SettingError {
    pub fn storage(error: impl ToString) -> Self {
        Self::Storage(error.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyChord {
    key: String,
    modifiers: Modifiers,
}

impl KeyChord {
    pub fn new(key: impl AsRef<str>, modifiers: Modifiers) -> Result<Self, SettingError> {
        let key = key.as_ref().trim().to_ascii_lowercase();

        if key.is_empty() {
            return Err(SettingError::InvalidKey(key));
        }

        Ok(Self { key, modifiers })
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn modifiers(&self) -> Modifiers {
        self.modifiers
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShortcutBinding {
    Disabled,
    Assigned { chord: KeyChord },
}

impl ShortcutBinding {
    pub fn assigned(key: impl AsRef<str>, modifiers: Modifiers) -> Result<Self, SettingError> {
        Ok(Self::Assigned {
            chord: KeyChord::new(key, modifiers)?,
        })
    }

    pub fn normalized(&self) -> Result<Self, SettingError> {
        match self {
            Self::Disabled => Ok(Self::Disabled),
            Self::Assigned { chord } => Self::assigned(chord.key(), chord.modifiers()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ShortcutChange {
    Set {
        action: ShortcutAction,
        binding: ShortcutBinding,
    },
    Reset {
        action: ShortcutAction,
    },
}
pub struct ShortcutItem {
}