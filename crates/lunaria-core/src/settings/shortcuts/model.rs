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
    /// Effective bindings after merging defaults and saved overrides; None disables the action.
    pub custom: Option<Vec<Shortcut>>,
}

impl ShortcutItem {
    pub fn is_modified(&self) -> bool {
        self.custom.as_deref().unwrap_or(&[]) != self.definition.defaults.as_slice()
    }
}

pub type ShortcutOverrides = HashMap<ShortcutAction, Option<Vec<Shortcut>>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persisted_gpui_modifiers_remain_compatible() {
        let saved: serde_json::Value = serde_json::from_str(
            r#"{"open_settings":[{"key":",","modifiers":{"control":false,"alt":false,"shift":false,"platform":true,"function":false}}],"quit":null}"#,
        )
        .unwrap();
        let overrides: ShortcutOverrides = serde_json::from_value(saved.clone()).unwrap();
        assert_eq!(serde_json::to_value(overrides).unwrap(), saved);

        let modifiers: ShortcutModifiers = serde_json::from_str(r#"{"platform":true}"#).unwrap();
        assert_eq!(
            modifiers,
            ShortcutModifiers {
                platform: true,
                ..ShortcutModifiers::default()
            }
        );
    }

    #[test]
    fn modified_state_compares_effective_bindings_including_disabled_actions() {
        let default = Shortcut::new(",", ShortcutModifiers::secondary_key());
        let mut item = ShortcutItem {
            definition: ShortcutDefinition {
                action: ShortcutAction::OpenSettings,
                title: "打开设置窗口",
                category: ShortcutCategory::Application,
                defaults: vec![default.clone()],
            },
            custom: Some(vec![default]),
        };
        assert!(!item.is_modified());
        item.custom = None;
        assert!(item.is_modified());
        item.custom = Some(vec![]);
        assert!(item.is_modified());
        item.custom = Some(vec![Shortcut::new("s", ShortcutModifiers::secondary_key())]);
        assert!(item.is_modified());
        item.definition.defaults.clear();
        item.custom = None;
        assert!(!item.is_modified());
    }
}
