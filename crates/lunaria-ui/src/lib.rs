rust_i18n::i18n!("../lunaria-assets/assets/locales", fallback = "en");

pub mod components;
pub mod extensions;
pub mod i18n;
pub mod theme;

pub use theme::ThemeManager;
