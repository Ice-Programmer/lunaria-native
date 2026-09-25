use gpui_kit::base::h_flex;
use gpui_kit::*;

use super::components::LaunchSidebar;
use super::router::{LauncherRoute, LauncherRouter};

const LAUNCHER_WIDTH: f32 = 800.0;
const LAUNCHER_HEIGHT: f32 = 580.0;
const LAUNCHER_MIN_WIDTH: f32 = LAUNCHER_WIDTH;
const LAUNCHER_MIN_HEIGHT: f32 = 400.0;

pub struct LaunchPage {
    current_route: LauncherRoute,
    current_view: AnyView,
    _route_subscription: Subscription,
}

impl LaunchPage {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let current_route = LauncherRouter::current(cx);
        let current_view = current_route.build(window, cx);

        let route_subscription =
            cx.observe_global_in::<LauncherRouter>(window, |this, window, cx| {
                let next_route = LauncherRouter::current(cx);

                if this.current_route == next_route {
                    return;
                }

                window.blur(cx);

                this.current_view = next_route.build(window, cx);
                this.current_route = next_route;

                cx.notify();
            });

        Self {
            current_route,
            current_view,
            _route_subscription: route_subscription,
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
                title: Some("Welcome to Lunaria".into()),
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
            .child(div().flex_1().h_full().child(self.current_view.clone()))
    }
}

