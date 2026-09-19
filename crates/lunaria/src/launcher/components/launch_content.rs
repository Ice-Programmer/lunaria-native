use gpui_kit::assets::IconName;
use gpui_kit::base::{h_flex, v_flex};
use gpui_kit::component::input::{Input, InputState};
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

    pub fn render_header(&self) -> impl IntoElement {
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
                    .w(px(200.))
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
                            .prefix(IconName::Search)
                            .h(px(36.)),
                    ),
            )
    }
}

impl Render for LaunchContent {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .min_w_0()
            .p_6()
            .child(self.render_header())
    }
}
