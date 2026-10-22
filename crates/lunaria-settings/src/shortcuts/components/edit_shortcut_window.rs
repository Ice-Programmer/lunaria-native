use crate::shortcuts::components::shortcut_keycaps::ShortcutKeycaps;
use crate::shortcuts::i18n::definition_i18n::ShortcutDefinitionI18nExt;
use gpui_kit::base::{h_flex, v_flex};
use gpui_kit::component::button::{Button, ButtonCustomVariant, ButtonVariants};
use gpui_kit::component::{ActiveTheme, Disableable, IconName, Sizable};
use gpui_kit::gpui::actions;
use gpui_kit::*;
use lunaria_app::windows::custom_window::{self, WindowConfig};
use lunaria_core::settings::shortcuts::model::ShortcutItem;
use lunaria_ui::i18n::keys;
use lunaria_ui::i18n_text;

actions!(shortcut_editor, [CloseShortcutEditor]);

const SHORTCUT_WINDOW_WIDTH: f32 = 300.;
const SHORTCUT_WINDOW_HEIGHT: f32 = 280.;

pub struct EditShortcutWindow {
    shortcut: ShortcutItem,
}

impl EditShortcutWindow {
    fn new(shortcut: ShortcutItem) -> Self {
        Self { shortcut }
    }

    pub fn open(shortcut: ShortcutItem, cx: &mut App) {
        custom_window::open(
            shortcut.definition.action,
            WindowConfig {
                title: Some(Self::title(&shortcut)),
                size: size(px(SHORTCUT_WINDOW_WIDTH), px(SHORTCUT_WINDOW_HEIGHT)),
                min_size: size(px(SHORTCUT_WINDOW_WIDTH), px(SHORTCUT_WINDOW_HEIGHT)),
            },
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
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .child("点击后按下组合键"),
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

    fn render_add_shortcut_btn(&self, cx: &App) -> Button {
        Button::new("add-shortcut")
            .icon(IconName::Plus)
            .ghost()
            .small()
            .self_start()
            .text_color(cx.theme().muted_foreground)
            .child("添加快捷键")
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
        v_flex()
            .px_4()
            .gap_2()
            .child(self.render_operation_line(cx))
            .child(self.render_binding_content(cx))
            .child(self.render_add_shortcut_btn(cx))
    }
}
