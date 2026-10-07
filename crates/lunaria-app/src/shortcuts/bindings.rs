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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::{AsKeystroke, actions};
    use lunaria_core::settings::shortcuts::catalog::default_shortcuts;

    actions!(binding_tests, [UnrelatedAction]);

    #[test]
    fn reapply_replaces_all_global_actions_and_preserves_other_bindings() {
        let mut items: Vec<_> = default_shortcuts()
            .into_iter()
            .map(|definition| ShortcutItem {
                custom: Some(definition.defaults.clone()),
                definition,
            })
            .collect();
        let initial = refreshed_bindings(
            &items,
            [
                KeyBinding::new("ctrl-a", UnrelatedAction, None),
                KeyBinding::new("ctrl-b", OpenSettings, Some("Editor")),
            ],
        );

        for item in &mut items {
            if item.definition.action == ShortcutAction::Quit {
                item.custom = None;
            } else {
                item.custom.as_mut().unwrap()[0].key = "F6".to_owned();
            }
        }
        let refreshed = refreshed_bindings(&items, initial);
        let refreshed = refreshed_bindings(&items, refreshed);

        assert_eq!(refreshed.len(), 5);
        assert_eq!(
            refreshed
                .iter()
                .filter(|binding| binding.action().as_any().is::<UnrelatedAction>())
                .count(),
            1
        );
        assert_eq!(
            refreshed
                .iter()
                .filter(|binding| binding.predicate().is_some())
                .count(),
            1
        );
        assert!(
            !refreshed
                .iter()
                .any(|binding| binding.action().as_any().is::<Quit>())
        );

        for binding in refreshed.iter().filter(|binding| {
            binding.predicate().is_none() && !binding.action().as_any().is::<UnrelatedAction>()
        }) {
            assert_eq!(binding.keystrokes().len(), 1);
            assert_eq!(binding.keystrokes()[0].as_keystroke().key, "f6");
        }
    }
}
