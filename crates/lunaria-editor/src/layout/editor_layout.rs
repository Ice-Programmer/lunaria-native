use gpui_kit::base::{h_flex, v_flex};
use gpui_kit::component::ActiveTheme;
use gpui_kit::{AnyElement, App, IntoElement, ParentElement, RenderOnce, Styled, Window, div};

#[derive(IntoElement)]
pub struct EditorLayout {
    title_bar: AnyElement,
    sidebar: AnyElement,
    content: AnyElement,
    status_bar: AnyElement,
}

impl EditorLayout {
    pub fn new(
        title_bar: impl IntoElement,
        sidebar: impl IntoElement,
        content: impl IntoElement,
        status_bar: impl IntoElement,
    ) -> Self {
        Self {
            title_bar: title_bar.into_any_element(),
            sidebar: sidebar.into_any_element(),
            content: content.into_any_element(),
            status_bar: status_bar.into_any_element(),
        }
    }
}

impl RenderOnce for EditorLayout {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(
                // 顶部栏
                div().w_full().flex_shrink_0().child(self.title_bar),
            )
            .child(
                // 侧边栏 + 内容
                h_flex()
                    .items_stretch()
                    .flex_1()
                    .overflow_hidden()
                    .child(self.sidebar)
                    .child(div().flex_1().overflow_hidden().child(self.content)),
            )
            .child(div().h_6().flex_shrink_0().child(self.status_bar))
    }
}
