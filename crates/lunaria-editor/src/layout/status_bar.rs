use gpui_kit::{Context, IntoElement, Render, Window, div};

pub struct EditorStatusBar;

impl Render for EditorStatusBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}
