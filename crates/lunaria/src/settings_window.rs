use crate::app::window_root::create_window_root;
use gpui_kit::*;
use lunaria_settings::setting_page::SettingPage;

pub fn open_settings(cx: &mut App) {
    cx.defer(|cx| {
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(
                size(px(800.), px(600.)),
                cx,
            )),
            window_min_size: Some(size(px(800.), px(600.))),
            titlebar: Some(TitlebarOptions {
                title: Some("Lunaria 设置".into()),
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