use gpui_kit::component::TITLE_BAR_HEIGHT;
use gpui_kit::{Context, IntoElement, Render, Styled, Window, div, px};
use lunaria_ui::components::placeholder::Placeholder;

const STATUS_BAR_HEIGHT: f32 = 24.;

pub struct EditorStatusBar;

impl Render for EditorStatusBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Placeholder::new("状态栏").w_full().h(px(STATUS_BAR_HEIGHT))
    }
}
