use gpui_kit::{Context, IntoElement, Render, Styled, Window};
use lunaria_ui::components::placeholder::Placeholder;

pub struct ScriptPage;

impl ScriptPage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for ScriptPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Placeholder::new("Script").size_full()
    }
}
