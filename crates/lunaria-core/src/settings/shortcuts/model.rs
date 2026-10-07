use gpui_kit::Modifiers;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Deserialize, Serialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ShortcutAction {
    OpenSettings,
    ToggleTheme,
    CloseWindow,
    Quit,
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
        self.custom
            .as_ref()
            .is_some_and(|custom| custom != &self.definition.defaults)
    }
}

pub type ShortcutOverrides = HashMap<ShortcutAction, Option<Vec<Shortcut>>>;
