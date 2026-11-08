use gpui_kit::{div, Context, IntoElement, Render, Window};

pub struct EditorSidebar;

impl Render for EditorSidebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}