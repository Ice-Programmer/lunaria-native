use gpui_kit::assets::IconName;
use gpui_kit::base::h_flex;
use gpui_kit::base::input::InputState;
use gpui_kit::component::{ActiveTheme, Icon, v_flex};
use gpui_kit::{
    AppContext, Context, Div, Entity, FontWeight, IntoElement, ParentElement, Render, Styled,
    Window, div,
};
use lunaria_ui::components::custom_input::CustomInput;
use lunaria_ui::extensions::input_18n::InputI18nExt;
use lunaria_ui::extensions::space_ext;
use lunaria_ui::extensions::space_ext::SpaceExt;
use lunaria_ui::i18n::keys;
use lunaria_ui::i18n_text;

pub struct CharacterSidebar {
    search_input: Entity<InputState>,
}

impl CharacterSidebar {
    pub fn new(cx: &mut Context<Self>, window: &mut Window) -> Self {
        let search_input = cx.new(|cx| {
            InputState::new(window, cx).placeholder_i18n(
                keys::editor::character::SEARCH_PLACEHOLDER,
                "搜索角色",
                window,
                cx,
            )
        });

        Self { search_input }
    }

    fn render_title(cx: &mut Context<Self>) -> Div {
        h_flex()
            .justify_between()
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(i18n_text!(keys::editor::menu::CHARACTER, "角色")),
                    )
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

    fn render_search(&self, cx: &mut Context<Self>) -> impl IntoElement {
        CustomInput::new(&self.search_input)
            .id("search-character")
            .prefix(IconName::Search)
            .w_full()
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
            .child(10.vertical_space())
            .child(self.render_search(cx))
    }
}
