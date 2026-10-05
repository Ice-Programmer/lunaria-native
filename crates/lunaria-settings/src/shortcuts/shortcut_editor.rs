use gpui_kit::component::input::InputState;
use gpui_kit::*;

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

    pub fn render_header() -> impl IntoElement {
        div()
    }
}

impl Render for ShortcutEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("hihi")
    }
}
