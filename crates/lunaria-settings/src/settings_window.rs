use crate::setting_page::SettingPage;
use gpui_kit::*;
use lunaria_app::windows::custom_window;
use lunaria_app::windows::custom_window::WindowConfig;

const SETTING_WINDOW_HEIGHT: f32 = 600.;
const SETTING_WINDOW_WIDTH: f32 = 820.;

pub fn open_settings(cx: &mut App) {
    custom_window::open(
        (),
        WindowConfig {
            title: None,
            size: size(px(SETTING_WINDOW_WIDTH), px(SETTING_WINDOW_HEIGHT)),
            ..Default::default()
        },
        cx,
        SettingPage::new,
    )
}
