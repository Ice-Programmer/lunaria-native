use crate::settings::shortcuts::model::{Shortcut, ShortcutAction, ShortcutDefinition};
use gpui_kit::Modifiers;

pub fn default_shortcuts() -> Vec<ShortcutDefinition> {
    // mac: command, windows: ctrl
    let command = Modifiers::secondary_key();

    vec![ShortcutDefinition {
        action: ShortcutAction::ToggleTheme,
        title: "",
        defaults: vec![Shortcut::new(",", command)],
    }]
}
