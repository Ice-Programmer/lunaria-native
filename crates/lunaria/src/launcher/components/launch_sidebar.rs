use gpui_kit::assets::IconName;
use gpui_kit::base::{h_flex, v_flex};
use gpui_kit::component::button::*;
use gpui_kit::component::{ActiveTheme, Icon};
use gpui_kit::*;

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

    fn render_action(_: &mut App) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(
                Button::new("create-project")
                    .mt_8()
                    .label("创建项目")
                    .primary()
                    .w_full()
                    .h(px(36.))
                    .icon(IconName::Plus),
            )
            .child(
                Button::new("open-project")
                    .outline()
                    .label("打开项目")
                    .icon(IconName::FolderOpen)
                    .h(px(36.))
                    .w_full(),
            )
    }

    fn action_item(
        cx: &mut App,
        icon: IconName,
        label: &'static str,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> impl IntoElement {
        Button::new(label)
            .ghost()
            .w_full()
            .h(px(32.))
            .child(
                h_flex()
                    .w_full()
                    .items_center()
                    .gap_4()
                    .child(
                        Icon::new(icon)
                            .size_5()
                            .mt(px(1.))
                            .text_color(cx.theme().muted_foreground),
                    )
                    .child(
                        div()
                            .text_base()
                            .child(label)
                            .text_color(cx.theme().muted_foreground),
                    ),
            )
            .on_click(on_click)
    }

    // todo need add action
    fn render_action_bottom(cx: &mut App) -> impl IntoElement {
        v_flex()
            .gap_2()
            .mb_4()
            .child(Self::action_item(
                cx,
                IconName::BookOpen,
                "使用指南",
                |_, _, _| {
                    println!("open user guide");
                },
            ))
            .child(Self::action_item(
                cx,
                IconName::MessageCircle,
                "问题反馈",
                |_, _, _| {
                    println!("open feedback");
                },
            ))
            .child(Self::action_item(
                cx,
                IconName::Github,
                "GitHub",
                |_, _, _| {
                    println!("open github");
                },
            ))
            .child(Self::action_item(
                cx,
                IconName::Info,
                "v0.1.0",
                |_, _, _| {
                    println!("open version");
                },
            ))
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
            .child(div().flex_1())
            .child(Self::render_action_bottom(cx))
    }
}
