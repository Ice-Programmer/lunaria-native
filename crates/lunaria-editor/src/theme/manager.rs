use std::path::PathBuf;

use gpui_kit::{
    App,
    component::{Theme, ThemeRegistry},
};

use crate::theme::error::ThemeError;

pub struct ThemeManager;

impl ThemeManager {
    pub const LIGHT: &'static str = "Lunaria Light";
    pub const DARK: &'static str = "Lunaria Dark";

    pub fn init(cx: &mut App) -> Result<(), ThemeError> {
        let theme_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/theme/themes");

        ThemeRegistry::watch_dir(theme_dir, cx, |cx| {
            if let Err(err) = Self::set_theme(Self::LIGHT, cx) {
                eprintln!("Failed to reload theme: {err}")
            }
        })
        .map_err(|err| ThemeError::LoadFailed {
            err: err.to_string(),
        })?;

        Ok(())
    }

    pub fn set_theme(name: &str, cx: &mut App) -> Result<(), ThemeError> {
        let config = ThemeRegistry::global(cx)
            .themes()
            .get(name)
            .cloned()
            .ok_or_else(|| ThemeError::NotFound {
                name: name.to_string(),
            })?;

        Theme::global_mut(cx).apply_config(&config);

        Theme::sync_base(cx);

        cx.refresh_windows();

        Ok(())
    }

    pub fn use_light(cx: &mut App) -> Result<(), ThemeError> {
        Self::set_theme(Self::LIGHT, cx)
    }

    pub fn use_dark(cx: &mut App) -> Result<(), ThemeError> {
        Self::set_theme(Self::DARK, cx)
    }

    pub fn toggle(cx: &mut App) -> Result<(), ThemeError> {
        if Theme::global(cx).is_dark() {
            Self::use_light(cx)
        } else {
            Self::use_dark(cx)
        }
    }
}
