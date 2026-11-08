use gpui_kit::component::TITLE_BAR_HEIGHT;
use gpui_kit::{Context, IntoElement, Render, Styled, Window, div};
use lunaria_ui::components::placeholder::Placeholder;

pub struct EditorTitleBar;

impl Render for EditorTitleBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Placeholder::new("顶栏").w_full().h(TITLE_BAR_HEIGHT)
    }
}
