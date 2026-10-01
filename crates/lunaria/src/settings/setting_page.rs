use gpui_kit::WindowHandle;
use gpui_kit::component::{ActiveTheme, Root};
use gpui_kit::*;

const SETTING_WIDTH: f32 = 800.;

const SETTING_HEIGHT: f32 = 600.;

struct SettingsWindow(WindowHandle<Root>);

impl Global for SettingsWindow {}

pub struct SettingsPage;

impl SettingsPage {
    pub fn open(cx: &mut App) {
        cx.defer(|cx| {
            // 如果已经打开，将窗口置顶
            if let Some(handle) = cx.try_global::<SettingsWindow>().map(|state| state.0) {
                if handle
                    .update(cx, |_, window, _| window.activate_window())
                    .is_ok()
                {
                    return;
                }
            }

            let options = WindowOptions {
                window_bounds: Some(WindowBounds::centered(
                    size(px(SETTING_WIDTH), px(SETTING_HEIGHT)),
                    cx,
                )),
                window_min_size: Some(size(px(SETTING_WIDTH), px(SETTING_HEIGHT))),
                titlebar: Some(TitlebarOptions {
                    appears_transparent: true,
                    ..Default::default()
                }),
                ..Default::default()
            };

            match cx.open_window(options, |window, cx| {
                let view = cx.new(|_| Self);

                cx.new(|cx| Root::new(view, window, cx))
            }) {
                Ok(handle) => cx.set_global(SettingsWindow(handle)),
                Err(error) => {
                    eprintln!("Failed to open settings window: {error}");
                }
            }
        })
    }
}

impl Render for SettingsPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().bg(cx.theme().background)
    }
}
