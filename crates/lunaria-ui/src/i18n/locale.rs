use gpui_kit::{App, Global, SharedString};

#[derive(Clone, Debug)]
pub struct LocaleState {
    pub locale: String,
}

impl Global for LocaleState {}

pub fn init(locale: &str, cx: &mut App) {
    gpui_kit::component::set_locale(locale);

    cx.set_global(LocaleState {
        locale: locale.to_owned(),
    });
}

pub fn set_locale(locale: &str, cx: &mut App) {
    if cx.global::<LocaleState>().locale == locale {
        return;
    }

    gpui_kit::component::set_locale(locale);

    cx.global_mut::<LocaleState>().locale = locale.to_owned();
    cx.refresh_windows();
}

pub fn i18n_text(key: &str, default_text: &str) -> SharedString {
    let locale = rust_i18n::locale();

    match crate::_rust_i18n_try_translate(&locale, key) {
        Some(translated) => translated.into_owned().into(),
        None => default_text.to_owned().into(),
    }
}

pub fn i18n_text_with_args(key: &str, default_text: &str, args: &[(&str, String)]) -> SharedString {
    let translated = i18n_text(key, default_text);

    let (names, values): (Vec<&str>, Vec<String>) = args.iter().cloned().unzip();

    rust_i18n::replace_patterns(&translated, &names, &values).into()
}

#[macro_export]
macro_rules! i18n_text {
    ($key:expr, $default_text:expr $(,)?) => {
        $crate::i18n::locale::i18n_text($key, $default_text)
    };

    (
        $key:expr,
        $default_text:expr,
        $($name:ident = $value:expr),+
        $(,)?
    ) => {
        $crate::i18n::locale::i18n_text_with_args(
            $key,
            $default_text,
            &[
                $(
                    (stringify!($name), ($value).to_string())
                ),+
            ],
        )
    };
}
