use gpui_kit::{Context, IntoElement, Render, Styled, Window};
use lunaria_ui::components::placeholder::Placeholder;

pub struct CharacterSidebar;

impl CharacterSidebar {
    pub fn new() -> Self {
        Self
    }
}

impl Render for CharacterSidebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Placeholder::new("character sidebar").size_full()
    }
}
