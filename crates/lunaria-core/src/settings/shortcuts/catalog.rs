use crate::settings::shortcuts::model::{
    Shortcut, ShortcutAction, ShortcutCategory, ShortcutDefinition, ShortcutModifiers,
};

pub fn default_shortcuts() -> Vec<ShortcutDefinition> {
    // mac: command, windows: ctrl
    let command = ShortcutModifiers::secondary_key();

    let shift_command = ShortcutModifiers {
        shift: true,
        ..command
    };

    vec![
        ShortcutDefinition {
            action: ShortcutAction::ToggleTheme,
            title: "切换明暗主题",
            category: ShortcutCategory::Application,
            defaults: vec![Shortcut::new("T", shift_command)],
        },
        ShortcutDefinition {
            action: ShortcutAction::OpenSettings,
            title: "打开设置窗口",
            category: ShortcutCategory::Application,
            defaults: vec![Shortcut::new(",", command)],
        },
        ShortcutDefinition {
            action: ShortcutAction::CloseWindow,
            title: "关闭当前窗口",
            category: ShortcutCategory::Application,
            defaults: vec![Shortcut::new("w", command)],
        },
        ShortcutDefinition {
            action: ShortcutAction::Quit,
            title: "退出应用",
            category: ShortcutCategory::Application,
            defaults: vec![Shortcut::new("q", command)],
        },
    ]
}
