use crate::pages::character::components::character_sidebar::CharacterSidebar;
use gpui_kit::component::h_flex;
use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px,
};
use lunaria_ui::components::placeholder::Placeholder;

pub struct CharacterPage {
    sidebar: Entity<CharacterSidebar>,
}

const CHARACTER_SIDEBAR_WIDTH: f32 = 200.0;

impl CharacterPage {
    pub fn new(cx: &mut Context<Self>, window: &mut Window) -> Self {
        Self {
            sidebar: cx.new(|cx| CharacterSidebar::new(cx, window)),
        }
    }
}

impl Render for CharacterPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .relative()
            .size_full()
            .child(
                div()
                    .h_full()
                    .w(px(CHARACTER_SIDEBAR_WIDTH))
                    .child(self.sidebar.clone()),
            )
            .child(
                div()
                    .h_full()
                    .flex_1()
                    .child(Placeholder::new("character content").size_full()),
            )
    }
}
