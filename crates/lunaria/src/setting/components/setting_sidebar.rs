use gpui_kit::assets::IconName;
use gpui_kit::base::input::InputState;
use gpui_kit::base::v_flex;
use gpui_kit::component::ActiveTheme;
use gpui_kit::*;
use lunaria_ui::components::custom_input::CustomInput;

pub struct SettingSidebar {
    search_input: Entity<InputState>,
}

impl SettingSidebar {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("搜索设置")
                .default_value("")
        });

        Self { search_input }
    }

    fn render_search_input(&self) -> CustomInput {
        CustomInput::new(&self.search_input)
            .id("search-setting")
            .aria_label("search settings")
            .prefix(IconName::Search)
            .mt_6()
    }
}

impl Render for SettingSidebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .px_6()
            .py_4()
            .border_color(cx.theme().sidebar_border)
            .bg(cx.theme().sidebar)
            .child(self.render_search_input())
    }
}
