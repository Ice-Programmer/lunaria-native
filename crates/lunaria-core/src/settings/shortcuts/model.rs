use crate::settings::error::SettingError;
use gpui_kit::Modifiers;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Platform {
    MacOS,
    Windows,
}

impl Platform {
    pub fn current() -> Result<Self, SettingError> {
        match std::env::consts::OS {
            "macos" => Ok(Self::MacOS),
            "windows" => Ok(Self::Windows),
            other => Err(SettingError::UnsupportedPlatform(other.to_owned())),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Platform::MacOS => "macos",
            Platform::Windows => "windowns",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Deserialize, Serialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ShortcutAction {
    ToggleTheme,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Shortcut {
    pub key: String,
    pub modifiers: Modifiers,
}

impl Shortcut {
    pub fn new(key: impl Into<String>, modifiers: Modifiers) -> Self {
        Self {
            key: key.into(),
            modifiers,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ShortcutDefinition {
    pub action: ShortcutAction,
    pub title: &'static str,
    pub defaults: Vec<Shortcut>,
}

#[derive(Clone, Debug)]
pub struct ShortcutItem {
    pub definition: ShortcutDefinition,
    pub custom: Option<Vec<Shortcut>>,
}

impl ShortcutItem {
    pub fn is_modified(&self) -> bool {
        self.custom
            .as_ref()
            .is_some_and(|custom| custom != &self.definition.defaults)
    }
}

pub type ShortcutOverrides = HashMap<ShortcutAction, Option<Vec<Shortcut>>>;
