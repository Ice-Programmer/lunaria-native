use std::hash::Hash;
use gpui_kit::base::{v_flex};
use gpui_kit::component::ActiveTheme;
use gpui_kit::component::button::*;
use gpui_kit::*;
use gpui_kit::assets::IconName;

#[derive(IntoElement)]
pub struct LaunchSidebar;

impl LaunchSidebar {
    pub fn new() -> Self {
        Self
    }

    fn render_brand(cx: &mut App) -> impl IntoElement {
        v_flex()
            .items_start()
            .gap_0()
            .child(
                div()
                    .text_3xl()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("Lunaria"),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .child("视觉小说创作空间"),
            )
    }

    fn render_action(cx: &mut App) -> impl IntoElement {
        v_flex().gap_3()
            .child(
                Button::new("create-project")
                    .mt_8()
                    .label("创建项目")
                    .primary()
                    .w_full()
                    .h(px(36.))
                    .icon(IconName::Plus)
            )
            .child(
                Button::new("open-project")
                    .outline()
                    .label("打开项目")
                    .icon(IconName::FolderOpen)
                    .h(px(36.))
                    .w_full()
            )
    }
}

impl RenderOnce for LaunchSidebar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .size_full()
            .px_6()
            .py_4()
            .border_r_1()
            .border_color(cx.theme().sidebar_border)
            .bg(cx.theme().sidebar)
            .child(Self::render_brand(cx))
            .child(Self::render_action(cx))
    }
}
