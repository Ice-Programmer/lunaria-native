use crate::settings::shortcuts::model::{Shortcut, ShortcutAction, ShortcutDefinition};
use gpui_kit::Modifiers;

pub fn default_shortcuts() -> Vec<ShortcutDefinition> {
    // mac: command, windows: ctrl
    let command = Modifiers::secondary_key();

    let shift_command = Modifiers {
        shift: true,
        ..command
    };

    vec![ShortcutDefinition {
        action: ShortcutAction::ToggleTheme,
        title: "切换明暗主题",
        defaults: vec![Shortcut::new("T", shift_command)],
    },
    ShortcutDefinition{
        action: ShortcutAction::OpenSettings,
        title: "打开设置窗口",
        defaults: vec![Shortcut::new(",", command)],
    }]
}
