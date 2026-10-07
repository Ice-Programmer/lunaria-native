use gpui_kit::{Keystroke, Modifiers};
use lunaria_core::settings::shortcuts::model::Shortcut;

pub fn to_keystroke(shortcut: &Shortcut) -> Keystroke {
    let modifiers = shortcut.modifiers;

    Keystroke {
        key: shortcut.key.clone(),
        modifiers: Modifiers {
            control: modifiers.control,
            alt: modifiers.alt,
            shift: modifiers.shift,
            platform: modifiers.platform,
            function: modifiers.function,
        },
        key_char: None,
    }
}
