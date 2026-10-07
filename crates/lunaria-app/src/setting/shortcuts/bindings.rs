use super::actions::{CloseWindow, OpenSettings, Quit, ToggleTheme};
use super::to_keystroke;
use gpui_kit::{App, KeyBinding};
use lunaria_core::settings::shortcuts::model::{ShortcutAction, ShortcutItem};
use lunaria_ui::ThemeManager;

pub fn init_bindings(items: &[ShortcutItem], open_settings: fn(&mut App), cx: &mut App) {
    apply(items, cx);

    cx.on_action(|_: &ToggleTheme, cx| {
        if let Err(error) = ThemeManager::toggle(cx) {
            eprintln!("Failed to toggle theme: {error}");
        }
    });

    cx.on_action(move |_: &OpenSettings, cx| {
        open_settings(cx);
    });

    cx.on_action(|_: &CloseWindow, cx| {
        if let Some(handle) = cx.active_window() {
            cx.defer(move |cx| {
                let _ = handle.update(cx, |_, window, _| {
                    window.remove_window();
                });
            });
        }
    });

    cx.on_action(|_: &Quit, cx| {
        cx.quit();
    });
}

pub fn apply(items: &[ShortcutItem], cx: &mut App) {
    let bindings = {
        let keymap = cx.key_bindings();
        let keymap = keymap.borrow();
        refreshed_bindings(items, keymap.bindings().cloned())
    };

    cx.clear_key_bindings();
    cx.bind_keys(bindings);
}

fn refreshed_bindings(
    items: &[ShortcutItem],
    existing: impl IntoIterator<Item = KeyBinding>,
) -> Vec<KeyBinding> {
    let mut bindings: Vec<_> = existing
        .into_iter()
        .filter(|binding| {
            let action = binding.action().as_any();
            let managed = action.is::<ToggleTheme>()
                || action.is::<OpenSettings>()
                || action.is::<CloseWindow>()
                || action.is::<Quit>();

            !(binding.predicate().is_none() && managed)
        })
        .collect();

    for item in items {
        for shortcut in item.custom.as_deref().unwrap_or(&[]) {
            let stroke = to_keystroke(shortcut).unparse();

            let binding = match item.definition.action {
                ShortcutAction::ToggleTheme => KeyBinding::new(&stroke, ToggleTheme, None),
                ShortcutAction::OpenSettings => KeyBinding::new(&stroke, OpenSettings, None),
                ShortcutAction::CloseWindow => KeyBinding::new(&stroke, CloseWindow, None),
                ShortcutAction::Quit => KeyBinding::new(&stroke, Quit, None),
            };

            bindings.push(binding);
        }
    }

    bindings
}
