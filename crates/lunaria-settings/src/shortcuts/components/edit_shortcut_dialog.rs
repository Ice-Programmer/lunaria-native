use crate::shortcuts::components::shortcut_keycaps::ShortcutKeycaps;
use gpui_kit::base::{h_flex, v_flex};
use gpui_kit::component::button::{Button, ButtonCustomVariant, ButtonVariants};
use gpui_kit::component::form::{Field, Form};
use gpui_kit::component::{ActiveTheme, Sizable, WindowExt};
use gpui_kit::{
    App, AppContext, Context, Div, FontWeight, IntoElement, ParentElement, Render, SharedString,
    Styled, Window, div, px,
};
use lunaria_core::settings::shortcuts::model::ShortcutItem;
use lunaria_ui::i18n::keys;
use lunaria_ui::i18n_text;

pub struct EditShortcutDialog {
    shortcut: ShortcutItem,
}

impl EditShortcutDialog {
    pub fn new(shortcut: ShortcutItem) -> Self {
        Self { shortcut }
    }

    pub fn open(shortcut: ShortcutItem, window: &mut Window, cx: &mut App) {
        let view = cx.new(|_| Self::new(shortcut));

        window.open_dialog(cx, move |dialog, _window, cx| {
            dialog
                .overlay_closable(false)
                .title(i18n_text!(
                    keys::setting::shortcut::edit_dialog::TITLE,
                    "编辑快捷键"
                ))
                .close_button(false)
                .child(view.clone())
                .footer(Self::render_footer())
        })
    }

    pub fn render_operation_line(&self, cx: &mut App) -> Div {
        h_flex()
            .justify_between()
            .child(
                div()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(self.shortcut.definition.title),
            )
            .child(
                Button::new(format!("reset-{}", self.shortcut.definition.title))
                    .ghost()
                    .cursor_pointer()
                    .small()
                    .text_color(cx.theme().muted_foreground)
                    .child("恢复默认"),
            )
    }

    pub fn render_binding_content(&self, cx: &App) -> Div {
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
            content = content.child(frame().child(ShortcutKeycaps::single(shortcut.clone())));
        }

        content
    }

    pub fn render_footer() -> Div {
        h_flex()
            .w_full()
            .justify_end()
            .gap_3()
            .child(
                Button::new("cancel-shortcut")
                    .child("取消")
                    .on_click(|_, window, cx| window.close_dialog(cx)),
            )
            .child(Button::new("submit-shortcut").primary().child("确认修改"))
    }
}

impl Render for EditShortcutDialog {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().w_full().pt_4().min_w_0().child(
            Form::horizontal()
                .label_width(px(60.))
                .gap_4()
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
                                .text_color(cx.theme().muted_foreground)
                                .font_weight(FontWeight::MEDIUM)
                                .child("快捷键")
                        })
                        .child(self.render_binding_content(cx)),
                ),
        )
    }
}
