use gpui_kit::base::StyledExt;
use gpui_kit::component::ActiveTheme;
use gpui_kit::*;

#[derive(IntoElement)]
pub struct Placeholder {
    label: SharedString,
    style: StyleRefinement,
}

impl Placeholder {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            style: StyleRefinement::default(),
        }
    }
}

impl Styled for Placeholder {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Placeholder {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_center()
            .border_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().muted)
            .text_color(cx.theme().muted_foreground)
            .text_sm()
            .refine_style(&self.style)
            .child(self.label)
    }
}
