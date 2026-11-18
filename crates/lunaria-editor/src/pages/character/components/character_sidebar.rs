use gpui_kit::assets::IconName;
use gpui_kit::base::h_flex;
use gpui_kit::component::{ActiveTheme, Icon, v_flex};
use gpui_kit::{Context, Div, FontWeight, IntoElement, ParentElement, Render, Styled, Window, div};

pub struct CharacterSidebar;

impl CharacterSidebar {
    pub fn new() -> Self {
        Self
    }

    fn render_title(cx: &mut Context<Self>) -> Div {
        h_flex()
            .justify_between()
            .child(
                h_flex()
                    .gap_2()
                    .child(div().font_weight(FontWeight::SEMIBOLD).child("角色"))
                    .child(
                        div()
                            .text_xs()
                            .items_end()
                            .text_color(cx.theme().muted_foreground)
                            .child("3"),
                    ),
            )
            .child(Icon::new(IconName::Plus))
    }
}

impl Render for CharacterSidebar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .px_4()
            .pt_4()
            .size_full()
            .border_l_1()
            .border_color(cx.theme().sidebar_border)
            .bg(cx.theme().sidebar)
            .child(Self::render_title(cx))
    }
}
