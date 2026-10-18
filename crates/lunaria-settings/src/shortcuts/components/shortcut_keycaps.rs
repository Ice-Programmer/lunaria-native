use gpui_kit::base::h_flex;
use gpui_kit::component::ActiveTheme;
use gpui_kit::component::kbd::Kbd;
use gpui_kit::{
    App, Div, IntoElement, Keystroke, ParentElement, RenderOnce, StyleRefinement, Styled, Window,
    div,
};
use lunaria_core::settings::shortcuts::model::Shortcut;

#[derive(IntoElement)]
pub struct ShortcutKeycaps {
    base: Div,
    shortcuts: Vec<Shortcut>,
}

impl ShortcutKeycaps {
    pub fn new(shortcuts: Option<Vec<Shortcut>>) -> Self {
        Self {
            base: h_flex().gap_1(),
            shortcuts: shortcuts.unwrap_or_default(),
        }
    }

    pub fn single(shortcut: Shortcut) -> Self {
        Self::new(Some(vec![shortcut]))
    }

    fn render_single_keycap(key: &str, cx: &App) -> Kbd {
        Kbd::new(Keystroke {
            modifiers: Default::default(),
            key: key.to_owned(),
            key_char: None,
        })
            .outline()
            .bg(cx.theme().muted)
            .text_color(cx.theme().foreground)
            .text_sm()
            .items_center()
            .justify_between()
            .flex()
            .whitespace_nowrap()
            .flex_shrink_0()
    }
}

impl Styled for ShortcutKeycaps {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for ShortcutKeycaps {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let mut group = self.base;

        if self.shortcuts.is_empty() {
            return group
                .text_color(cx.theme().muted_foreground)
                .child("未设置");
        }

        for (index, shortcut) in self.shortcuts.iter().enumerate() {
            if index > 0 {
                group = group.child(
                    div()
                        .text_color(cx.theme().muted_foreground)
                        .child("或"),
                );
            }

            for key in shortcut.key_parts() {
                group = group.child(Self::render_single_keycap(key, cx));
            }
        }

        group
    }
}