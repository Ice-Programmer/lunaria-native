use gpui_kit::base::v_flex;
use gpui_kit::component::input::InputState;
use gpui_kit::*;

pub struct ShortcutContent {
    search_input: Entity<InputState>,
}

impl ShortcutContent {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("搜索快捷键")
                .default_value("")
        });

        Self { search_input }
    }

    pub fn render_header() -> impl IntoElement {
        div()
    }
}

impl Render for ShortcutContent {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().child("hihi")
    }
}
