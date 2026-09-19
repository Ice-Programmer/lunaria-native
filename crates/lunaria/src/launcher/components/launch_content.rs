use gpui_kit::assets::IconName;
use gpui_kit::base::{h_flex, v_flex};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::{ActiveTheme, Icon};
use gpui_kit::*;

pub struct LaunchContent {
    search_input: Entity<InputState>,
}

impl LaunchContent {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("搜索项目")
                .default_value("")
        });

        Self { search_input }
    }

    fn render_header(&self) -> impl IntoElement {
        h_flex()
            .w_full()
            .justify_between()
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("最近项目"),
            )
            .child(
                div()
                    .w(px(180.))
                    .on_mouse_down_out(|_, window, cx| {
                        window.blur(cx);
                    })
                    .capture_key_down(|event, window, cx| {
                        if event.keystroke.key == "escape" {
                            window.blur(cx);
                        }
                    })
                    .child(
                        Input::new(&self.search_input)
                            .id("search-project")
                            .aria_label("search recent project")
                            .prefix(IconName::Search),
                    ),
            )
    }

    fn render_empty_state(cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .flex_1()
            .items_center()
            .justify_center()
            .gap_3()
            .pb_8()
            .child(
                Icon::new(IconName::Folder)
                    .size(px(32.))
                    .text_color(cx.theme().muted_foreground),
            )
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("还没有最近项目"),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .child("新建一个故事，或打开已有的项目文件"),
            )
    }
}

impl Render for LaunchContent {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .min_w_0()
            .p_6()
            .child(self.render_header())
            .child(Self::render_empty_state(cx))
    }
}
