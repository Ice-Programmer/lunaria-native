use gpui_kit::App;
use lunaria_core::settings::language::model::Language;
use lunaria_ui::i18n::locale::LocaleState;

pub fn next_language(cx: &mut App) {
    let locale = &cx.global::<LocaleState>().locale;

    let index = Language::ALL
        .iter()
        .position(|item| item.locale() == locale.as_str())
        .unwrap_or(0);

    let next_index = (index + 1) % Language::ALL.len();

    let next_locale = &Language::ALL[next_index];
}

