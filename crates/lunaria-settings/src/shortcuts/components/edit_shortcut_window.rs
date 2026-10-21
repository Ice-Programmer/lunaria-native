use crate::shortcuts::components::shortcut_keycaps::ShortcutKeycaps;
use crate::shortcuts::i18n::definition_i18n::ShortcutDefinitionI18nExt;
use gpui_kit::base::{h_flex, v_flex};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::form::{Field, Form};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::{ActiveTheme, Disableable, Root, Sizable};
use gpui_kit::gpui::actions;
use gpui_kit::*;
use lunaria_app::windows::custom_window::{self, WindowConfig};
use lunaria_core::settings::shortcuts::model::{ShortcutAction, ShortcutItem};
use lunaria_ui::i18n::keys;
use lunaria_ui::i18n_text;
use std::collections::HashMap;

actions!(shortcut_editor, [CloseShortcutEditor]);

const SHORTCUT_WINDOW_WIDTH: f32 = 300.;
const SHORTCUT_WINDOW_HEIGHT: f32 = 280.;

#[derive(Default)]
struct ShortcutEditorWindows {
    windows: HashMap<ShortcutAction, WindowHandle<Root>>,
}

impl Global for ShortcutEditorWindows {}

pub struct EditShortcutWindow {
    shortcut: ShortcutItem,
}

impl EditShortcutWindow {
    fn new(shortcut: ShortcutItem) -> Self {
        Self { shortcut }
    }

    pub fn open(shortcut: ShortcutItem, window: &mut Window, cx: &mut App) {
        custom_window::open(
            shortcut.definition.action,
            WindowConfig {
                title: Some(Self::title(&shortcut)),
                size: size(px(SHORTCUT_WINDOW_WIDTH), px(SHORTCUT_WINDOW_HEIGHT)),
                min_size: size(px(SHORTCUT_WINDOW_WIDTH), px(SHORTCUT_WINDOW_HEIGHT)),
            },
            window,
            cx,
            move |_, _| Self::new(shortcut),
        )
    }

    fn title(shortcut: &ShortcutItem) -> SharedString {
        format!(
            "{} — {}",
            i18n_text!(keys::setting::shortcut::edit_dialog::TITLE, "编辑快捷键"),
            shortcut.definition.title_i18n()
        )
        .into()
    }

    fn render_operation_line(&self, cx: &App) -> Div {
        h_flex()
            .w_full()
            .min_w_0()
            .gap_3()
            .justify_between()
            .child(
                div()
                    .min_w_0()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(self.shortcut.definition.title_i18n()),
            )
            .child(
                Button::new("reset-shortcut")
                    .ghost()
                    .small()
                    .text_color(cx.theme().muted_foreground)
                    .label("恢复默认"),
            )
    }

    fn render_binding_content(&self, cx: &App) -> Div {
        let shortcuts = self.shortcut.custom.as_deref().unwrap_or(&[]);
        let frame = || {
            h_flex()
                .w_full()
                .min_w_0()
                .items_center()
                .px_3()
                .py_2()
                .border_1()
                .border_color(cx.theme().input)
                .rounded_lg()
                .bg(cx.theme().background)
        };

        let mut content = v_flex().w_full().min_w_0().gap_2();
        if shortcuts.is_empty() {
            return content.child(frame().child(ShortcutKeycaps::new(None)));
        }

        for shortcut in shortcuts {
            content =
                content.child(frame().child(ShortcutKeycaps::single(shortcut.clone()).flex_wrap()));
        }
        content
    }

    fn render_footer() -> Div {
        h_flex()
            .w_full()
            .flex_shrink_0()
            .justify_end()
            .gap_3()
            .p_4()
            .child(
                Button::new("cancel-shortcut")
                    .label("取消")
                    .on_click(|_, window, _| window.remove_window()),
            )
            .child(
                Button::new("submit-shortcut")
                    .primary()
                    .disabled(true)
                    .label("确认修改"),
            )
    }
}

impl Render for EditShortcutWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("shortcut-editor-body")
            .flex_1()
            .min_h_0()
            .overflow_y_scrollbar()
            .child(
                div().p_4().child(
                    Form::horizontal()
                        .label_width(px(60.))
                        .child(
                            Field::new()
                                .min_w_0()
                                .items_start()
                                .label_fn(|_, cx| {
                                    div()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(cx.theme().muted_foreground)
                                        .child("操作")
                                })
                                .child(self.render_operation_line(cx)),
                        )
                        .child(
                            Field::new()
                                .min_w_0()
                                .items_start()
                                .label_fn(|_, cx| {
                                    div()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(cx.theme().muted_foreground)
                                        .child("快捷键")
                                })
                                .child(self.render_binding_content(cx)),
                        ),
                ),
            )
    }
}
