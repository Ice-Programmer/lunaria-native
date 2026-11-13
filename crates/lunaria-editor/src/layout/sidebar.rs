use gpui_kit::{Context, IntoElement, Render, Styled, Window, div, px};
use lunaria_ui::components::placeholder::Placeholder;

const SIDEBAR_WIDTH: f32 = 40.0;

pub struct EditorSidebar;

impl Render for EditorSidebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Placeholder::new("侧边栏").h_full().w(px(SIDEBAR_WIDTH))
    }
}
