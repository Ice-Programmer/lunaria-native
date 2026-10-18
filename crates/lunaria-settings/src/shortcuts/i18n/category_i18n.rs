use gpui_kit::SharedString;
use lunaria_core::settings::shortcuts::model::ShortcutCategory;
use lunaria_ui::i18n::keys;
use lunaria_ui::i18n_text;

pub trait ShortcutCategoryI18nExt {
    fn label_i18n(self) -> SharedString;
    fn description_i18n(self) -> SharedString;
}

impl ShortcutCategoryI18nExt for ShortcutCategory {
    fn label_i18n(self) -> SharedString {
        let key = match self {
            Self::Application => keys::setting::APPLICATION,
            Self::Launcher => keys::setting::LAUNCHER,
            Self::Editor => keys::setting::EDITOR,
        };

        i18n_text!(key, self.label())
    }

    fn description_i18n(self) -> SharedString {
        let key = match self {
            Self::Application => keys::setting::shortcut::APPLICATION_DESCRIPTION,
            Self::Launcher => keys::setting::shortcut::LAUNCHER_DESCRIPTION,
            Self::Editor => keys::setting::shortcut::EDITOR_DESCRIPTION,
        };

        i18n_text!(key, self.description())
    }
}
