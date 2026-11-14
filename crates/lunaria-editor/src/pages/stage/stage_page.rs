use gpui_kit::{Context, IntoElement, Render, Styled, Window};
use lunaria_ui::components::placeholder::Placeholder;

pub struct StagePage;

impl StagePage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for StagePage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Placeholder::new("Stage").size_full()
    }
}
