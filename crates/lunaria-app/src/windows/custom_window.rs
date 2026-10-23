use crate::windows::window_root::create_window_root_with_fps;
use gpui_kit::base::v_flex;
use gpui_kit::component::notification::Notification;
use gpui_kit::component::{ActiveTheme, Root, TitleBar, WindowExt};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyView, App, AppContext, Bounds, Context, FontWeight, Global, IntoElement, ParentElement,
    Pixels, Render, SharedString, Size, Styled, TitlebarOptions, Window, WindowBounds,
    WindowHandle, WindowOptions, div, px,
};
use std::any::TypeId;
use std::collections::HashMap;
use std::hash::Hash;

pub struct WindowConfig {
    pub title: Option<SharedString>,
    pub size: Size<Pixels>,
    pub min_size: Option<Size<Pixels>>,
    pub need_minimizable: bool,
    pub need_fullscreen: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: None,
            size: Size::default(),
            min_size: None,
            need_minimizable: true,
            need_fullscreen: true,
        }
    }
}

// 全局管理页面
struct WindowRegistry<K> {
    windows: HashMap<(TypeId, K), WindowHandle<Root>>,
}

impl<K> Default for WindowRegistry<K> {
    fn default() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }
}

impl<K: 'static> Global for WindowRegistry<K> {}

struct WindowFrame {
    title: Option<SharedString>,
    content: AnyView,
}

impl Render for WindowFrame {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if cfg!(target_os = "macos") && self.title.is_none() {
            return div()
                .size_full()
                .bg(cx.theme().background)
                .child(self.content.clone());
        }

        let title = self
            .title
            .clone()
            .map(|text| div().text_sm().truncate().child(text));

        v_flex()
            .size_full()
            .bg(cx.theme().background)
            .child(
                TitleBar::new()
                    .bg(cx.theme().background)
                    .border_b_0()
                    .when(cfg!(target_os = "macos"), |bar| bar.pr(px(80.)))
                    .child(
                        div()
                            .flex()
                            .flex_1()
                            .min_w_0()
                            .h_full()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(cx.theme().muted_foreground)
                                    .children(title),
                            ),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .min_w_0()
                    .w_full()
                    .child(self.content.clone()),
            )
    }
}

pub fn open<K, V>(
    key: K,
    config: WindowConfig,
    cx: &mut App,
    build: impl FnOnce(&mut Window, &mut Context<V>) -> V + 'static,
) where
    K: Eq + Hash + 'static,
    V: Render + 'static,
{
    let source_window = cx.active_window();

    cx.defer(move |cx| {
        let key = (TypeId::of::<V>(), key);

        let existing = cx
            .default_global::<WindowRegistry<K>>()
            .windows
            .get(&key)
            .copied();

        if existing.is_some_and(|handle| {
            // 如果存在当前页面，激活页面置前
            handle
                .update(cx, |_, window, _| window.activate_window())
                .is_ok()
        }) {
            return;
        }

        let display_id = source_window.and_then(|handle| {
            handle
                .update(cx, |_, window, cx| {
                    window.display(cx).map(|display| display.id())
                })
                .ok()
                .flatten()
        });

        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                display_id,
                config.size,
                cx,
            ))),
            window_min_size: Some(config.min_size.unwrap_or(config.size)),
            titlebar: Some(TitlebarOptions {
                title: config.title.clone(),
                ..TitleBar::title_bar_options()
            }),
            display_id,
            app_owns_titlebar_drag: config.title.is_some(),
            is_minimizable: config.need_minimizable,
            is_resizable: config.need_fullscreen,
            ..TitleBar::window_options()
        };

        match cx.open_window(options, move |window, cx| {
            let content = cx.new(|cx| build(window, cx));

            let frame = cx.new(|_| WindowFrame {
                title: config.title,
                content: content.into(),
            });

            create_window_root_with_fps(frame, false, window, cx)
        }) {
            Ok(handle) => {
                cx.global_mut::<WindowRegistry<K>>()
                    .windows
                    .insert(key, handle);
            }
            Err(error) => {
                eprintln!("Failed to open window: {error}");

                if let Some(source_window) = source_window {
                    let _ = source_window.update(cx, |_, window, cx| {
                        window.push_notification(
                            Notification::error(format!("无法打开窗口：{error}")),
                            cx,
                        );
                    });
                }
            }
        }
    })
}
