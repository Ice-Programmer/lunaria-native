use gpui_kit::base::v_flex;
use gpui_kit::{
    Context, Element, FontWeight, IntoElement, ParentElement, Render, Styled, Window, div,
};

pub struct CreateProjectForm {}

impl CreateProjectForm {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {}
    }

    fn render_header_title(&self) -> impl IntoElement {
        div()
            .text_2xl()
            .font_weight(FontWeight::SEMIBOLD)
            .child("新建项目")
    }
}

impl Render for CreateProjectForm {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .min_w_0()
            .p_6()
            .child(self.render_header_title())
    }
}
