use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Deserialize, Serialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ShortcutAction {
    OpenSettings,
    ToggleTheme,
    CloseWindow,
    Quit,
    SwitchLanguage,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(default)]
pub struct ShortcutModifiers {
    pub control: bool,
    pub alt: bool,
    pub shift: bool,
    pub platform: bool,
    pub function: bool,
}

impl ShortcutModifiers {
    pub fn secondary_key() -> Self {
        Self {
            control: !cfg!(target_os = "macos"),
            platform: cfg!(target_os = "macos"),
            ..Self::default()
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Shortcut {
    pub key: String,
    pub modifiers: ShortcutModifiers,
}

impl Shortcut {
    pub fn new(key: impl Into<String>, modifiers: ShortcutModifiers) -> Self {
        Self {
            key: key.into(),
            modifiers,
        }
    }

    pub fn key_parts(&self) -> Vec<&str> {
        let modifiers = self.modifiers;
        let mut keys = Vec::new();

        for (enabled, key) in [
            (modifiers.control, "ctrl"),
            (modifiers.alt, "alt"),
            (modifiers.shift, "shift"),
            (modifiers.platform, "cmd"),
            (modifiers.function, "fn"),
        ] {
            if enabled {
                keys.push(key);
            }
        }
        keys.push(self.key.as_str());

        keys
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShortcutCategory {
    Application,
    Launcher,
    Editor,
}

impl ShortcutCategory {
    pub fn label(self) -> &'static str {
        match self {
            Self::Application => "应用",
            Self::Launcher => "欢迎页",
            Self::Editor => "编辑器",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Application => "应用相关操作",
            Self::Launcher => "项目创建与打开",
            Self::Editor => "内容编辑相关操作",
        }
    }
}

#[derive(Clone, Debug)]
pub struct ShortcutDefinition {
    pub action: ShortcutAction,
    pub title: &'static str,
    pub category: ShortcutCategory,
    pub defaults: Vec<Shortcut>,
}

#[derive(Clone, Debug)]
pub struct ShortcutItem {
    pub definition: ShortcutDefinition,
    pub custom: Option<Vec<Shortcut>>,
}

impl ShortcutItem {
    pub fn is_modified(&self) -> bool {
        self.custom.as_deref().unwrap_or(&[]) != self.definition.defaults.as_slice()
    }
}

pub type ShortcutOverrides = HashMap<ShortcutAction, Option<Vec<Shortcut>>>;
