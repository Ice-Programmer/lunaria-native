use gpui_kit::base::v_flex;
use gpui_kit::component::ActiveTheme;
use gpui_kit::*;

#[derive(IntoElement)]
pub struct SettingSidebar;

impl SettingSidebar {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for SettingSidebar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .size_full()
            .px_6()
            .py_4()
            .border_color(cx.theme().sidebar_border)
            .bg(cx.theme().sidebar)
    }
}
