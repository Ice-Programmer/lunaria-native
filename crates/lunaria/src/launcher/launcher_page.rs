use super::components::LaunchSidebar;
use gpui_kit::{base::h_flex, *};

const LAUNCHER_WIDTH: f32 = 800.0;
const LAUNCHER_HEIGHT: f32 = 580.0;
const LAUNCHER_MIN_WIDTH: f32 = LAUNCHER_WIDTH;
const LAUNCHER_MIN_HEIGHT: f32 = LAUNCHER_HEIGHT;

pub struct LaunchPage {
    title: SharedString,
}

impl LaunchPage {
    pub fn new() -> Self {
        Self {
            title: "Lunaria".into(),
        }
    }

    pub fn window_options(cx: &App) -> WindowOptions {
        WindowOptions {
            window_bounds: Some(WindowBounds::centered(
                size(px(LAUNCHER_WIDTH), px(LAUNCHER_HEIGHT)),
                cx,
            )),
            window_min_size: Some(size(px(LAUNCHER_MIN_WIDTH), px(LAUNCHER_MIN_HEIGHT))),
            titlebar: Some(TitlebarOptions {
                title: Some("Lunaria".into()),
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}

impl Render for LaunchPage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .size_full()
            .child(
                div()
                    .w(relative(0.25))
                    .h_full()
                    .flex_shrink_0()
                    .child(LaunchSidebar::new()),
            )
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .items_center()
                    .justify_center()
                    .child(self.title.clone()),
            )
    }
}
