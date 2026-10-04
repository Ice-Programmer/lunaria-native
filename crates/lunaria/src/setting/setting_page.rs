use crate::app::window_root::create_window_root;
use crate::setting::components::setting_sidebar::SettingSidebar;
use crate::setting::router::{SettingRoute, SettingRouter};
use gpui_kit::WindowHandle;
use gpui_kit::base::h_flex;
use gpui_kit::component::Root;
use gpui_kit::*;

const SETTING_WIDTH: f32 = 800.;
const SETTING_HEIGHT: f32 = 600.;
const SETTING_SIDEBAR_WIDTH: f32 = 200.0;

struct SettingsWindow(WindowHandle<Root>);

impl Global for SettingsWindow {}

pub struct SettingPage {
    sidebar: Entity<SettingSidebar>,
    current_route: SettingRoute,
    current_view: AnyView,
    _route_subscription: Subscription,
}

impl SettingPage {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let sidebar = cx.new(|cx| SettingSidebar::new(window, cx));
        let current_route = SettingRouter::current(cx);
        let current_view = current_route.build(window, cx);

        let route_subscription =
            cx.observe_global_in::<SettingRouter>(window, |this, window, cx| {
                let next_route = SettingRouter::current(cx);

                if this.current_route == next_route {
                    return;
                }

                window.blur(cx);

                this.current_view = next_route.build(window, cx);
                this.current_route = next_route;

                cx.notify();
            });

        Self {
            sidebar,
            current_route,
            current_view,
            _route_subscription: route_subscription,
        }
    }

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
                let view = cx.new(|cx| Self::new(window, cx));
                create_window_root(view, window, cx)
            }) {
                Ok(handle) => cx.set_global(SettingsWindow(handle)),
                Err(error) => {
                    eprintln!("Failed to open setting window: {error}");
                }
            }
        })
    }
}

impl Render for SettingPage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .size_full()
            .relative()
            .child(
                div()
                    .w(px(SETTING_SIDEBAR_WIDTH))
                    .h_full()
                    .flex_shrink_0()
                    .child(self.sidebar.clone()),
            )
            .child(div().flex_1().h_full().child(self.current_view.clone()))
    }
}
