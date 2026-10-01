use gpui_kit::{App, KeyBinding, Keystroke, actions};
use lunaria_core::settings::shortcuts::model::{ShortcutAction, ShortcutItem};
use lunaria_ui::ThemeManager;

actions!(lunaria, [ToggleTheme]);

pub fn init(items: &[ShortcutItem], cx: &mut App) {
    apply(items, cx);

    cx.on_action(|_: &ToggleTheme, cx| {
        if let Err(error) = ThemeManager::toggle(cx) {
            eprintln!("Failed to toggle theme: {error}");
        }
    });
}

pub fn apply(items: &[ShortcutItem], cx: &mut App) {
    let mut bindings: Vec<_> = cx
        .key_bindings()
        .borrow()
        .bindings()
        .filter(|binding| {
            !(binding.predicate().is_none() && binding.action().as_any().is::<ToggleTheme>())
        })
        .cloned()
        .collect();

    for item in items {
        let Some(shortcuts) = &item.custom else {
            continue;
        };

        for shortcut in shortcuts {
            let keystroke = Keystroke {
                key: shortcut.key.clone(),
                modifiers: shortcut.modifiers,
                key_char: None,
            };

            let binding = match item.definition.action {
                ShortcutAction::ToggleTheme => {
                    KeyBinding::new(&keystroke.unparse(), ToggleTheme, None)
                }
            };

            bindings.push(binding);
        }
    }

    cx.clear_key_bindings();
    cx.bind_keys(bindings);
}
