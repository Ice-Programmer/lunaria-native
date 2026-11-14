use gpui_kit::{Context, IntoElement, Render, Styled, Window, px};
use lunaria_ui::components::placeholder::Placeholder;

pub struct CharacterPage;

impl CharacterPage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for CharacterPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Placeholder::new("Character").size_full()
    }
}
