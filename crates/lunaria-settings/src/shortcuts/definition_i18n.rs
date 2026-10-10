use gpui_kit::SharedString;
use lunaria_core::settings::shortcuts::model::{ShortcutAction, ShortcutDefinition};
use lunaria_ui::i18n::keys;
use lunaria_ui::i18n_text;

pub trait ShortcutDefinitionI18nExt {
    fn title_i18n(&self) -> SharedString;
}

impl ShortcutDefinitionI18nExt for ShortcutDefinition {
    fn title_i18n(&self) -> SharedString {
        let key = match self.action {
            ShortcutAction::OpenSettings => keys::setting::shortcut::actions::OPEN_SETTINGS,
            ShortcutAction::ToggleTheme => keys::setting::shortcut::actions::TOGGLE_THEME,
            ShortcutAction::CloseWindow => keys::setting::shortcut::actions::CLOSE_WINDOW,
            ShortcutAction::Quit => keys::setting::shortcut::actions::QUIT,
            ShortcutAction::SwitchLanguage => keys::setting::shortcut::actions::SWITCH_LANGUAGE,
        };

        i18n_text!(key, self.title)
    }
}
