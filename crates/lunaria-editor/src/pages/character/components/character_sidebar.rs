use gpui_kit::{Context, Div, IntoElement, ParentElement, Render, Styled, Window};
use gpui_kit::assets::IconName;
use gpui_kit::base::h_flex;
use gpui_kit::component::{v_flex, Icon, ActiveTheme};
use lunaria_ui::components::placeholder::Placeholder;

pub struct CharacterSidebar;

impl CharacterSidebar {
    pub fn new() -> Self {
        Self
    }

    fn render_title() -> Div {
        h_flex()
            .justify_between()
            .child("角色")
            .child("3")
            .child(Icon::new(IconName::Plus))
    }
}

impl Render for CharacterSidebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .px_3()
            .pt_4()
            .size_full()
            .border_l_1()
            .border_color(cx.theme().sidebar_border)
            .bg(cx.theme().sidebar)
            .child(Self::render_title())
    }
}
