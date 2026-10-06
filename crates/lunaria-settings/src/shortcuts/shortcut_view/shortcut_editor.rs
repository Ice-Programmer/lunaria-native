use gpui_kit::assets::IconName;
use gpui_kit::base::{Disableable, h_flex};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::InputState;
use gpui_kit::component::{ActiveTheme, Selectable, Sizable};
use gpui_kit::*;
use lunaria_ui::components::custom_button::CustomButton;
use lunaria_ui::components::custom_input::CustomInput;

pub struct ShortcutEditor {
    search_input: Entity<InputState>,
}

impl ShortcutEditor {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("搜索快捷键")
                .default_value("")
        });

        Self { search_input }
    }

    pub fn render_header(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .child(
                CustomInput::new(&self.search_input)
                    .id("search-shortcuts")
                    .aria_label("search shortcut settings")
                    .prefix(IconName::Search)
                    .w(relative(0.3)),
            )
            .child(Button::new("search-shortcuts-commend").label("按键检索"))
            .child(
                Button::new("shortcut-filter-all")
                    .ghost()
                    .label("全部")
                    .child(div().text_color(cx.theme().muted_foreground).child("12")),
            )
            .child(
                Button::new("shortcut-filter-modify")
                    .ghost()
                    .label("已修改")
                    .child(div().text_color(cx.theme().muted_foreground).child("8")),
            )
            .child(div().flex_1())
            .child(
                Button::new("reset-shortcut-settings")
                    .ghost()
                    .label("恢复全部默认")
                    .icon(IconName::Undo2)
                    .disabled(true),
            )
    }
}

impl Render for ShortcutEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().child(self.render_header(cx))
    }
}
