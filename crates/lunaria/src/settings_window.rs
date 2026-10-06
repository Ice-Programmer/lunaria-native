use crate::app::window_root::create_window_root;
use gpui_kit::*;
use lunaria_settings::setting_page::SettingPage;

const SETTING_WINDOW_HEIGHT: f32 = 600.;
const SETTING_WINDOW_WIDTH: f32 = 820.;

pub fn open_settings(cx: &mut App) {
    cx.defer(|cx| {
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(
                size(px(SETTING_WINDOW_WIDTH), px(SETTING_WINDOW_HEIGHT)),
                cx,
            )),
            window_min_size: Some(size(px(SETTING_WINDOW_WIDTH), px(SETTING_WINDOW_HEIGHT))),
            titlebar: Some(TitlebarOptions {
                title: Some("Settings".into()),
                appears_transparent: true,
                ..Default::default()
            }),
            ..Default::default()
        };

        if let Err(error) = cx.open_window(options, |window, cx| {
            let view = cx.new(|cx| SettingPage::new(window, cx));
            create_window_root(view, window, cx)
        }) {
            eprintln!("Failed to open settings: {error}");
        }
    });
}
