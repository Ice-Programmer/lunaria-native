use crate::shortcuts::components::shortcut_keycaps::ShortcutKeycaps;
use crate::shortcuts::i18n::definition_i18n::ShortcutDefinitionI18nExt;
use gpui_kit::base::{h_flex, v_flex};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{
    ActiveTheme, ElementExt, IconName, Sizable, TITLE_BAR_HEIGHT,
};
use gpui_kit::gpui::actions;
use gpui_kit::*;
use lunaria_app::windows::custom_window::{self, WindowConfig};
use lunaria_core::settings::shortcuts::model::ShortcutItem;

actions!(shortcut_editor, [CloseShortcutEditor]);

const SHORTCUT_WINDOW_WIDTH: f32 = 260.;
const SHORTCUT_WINDOW_MIN_HEIGHT: f32 = 180.;
const SHORTCUT_ROW_HEIGHT: f32 = 32.;
const SHORTCUT_WINDOW_MAX_HEIGHT: f32 = 600.;
const SHORTCUT_FOOTER_HEIGHT: f32 = 56.;

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
                title: Some(format!("{}", shortcut.definition.title_i18n()).into()),
                size: size(px(SHORTCUT_WINDOW_WIDTH), px(SHORTCUT_WINDOW_MIN_HEIGHT)),
                min_size: size(px(SHORTCUT_WINDOW_WIDTH), px(SHORTCUT_WINDOW_MIN_HEIGHT)),
            },
            cx,
            move |_, _| Self::new(shortcut),
        )
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

    fn resize_to_fit_content(bounds: Bounds<Pixels>, window: &mut Window, cx: &mut App) {
        // 内容高度 + 自定义标题栏高度 + footer
        let height = (bounds.size.height + TITLE_BAR_HEIGHT + px(SHORTCUT_FOOTER_HEIGHT))
            .ceil()
            .clamp(
                px(SHORTCUT_WINDOW_MIN_HEIGHT),
                px(SHORTCUT_WINDOW_MAX_HEIGHT),
            );

        window.defer(cx, move |window, _| {
            if window.is_fullscreen() || window.is_maximized() {
                return;
            }

            let current = window.viewport_size();

            if (current.height - height).abs() >= px(1.) {
                window.resize(size(current.width, height))
            }
        })
    }

    fn render_footer() -> Div {
        h_flex()
            .w_full()
            .flex_shrink_0()
            .justify_end()
            .p_4()
            .gap_3()
            .child(
                Button::new("cancel-shortcut")
                    .label("取消")
                    .on_click(|_, window, _| window.remove_window()),
            )
            .child(Button::new("submit-shortcut").primary().label("保存"))
    }
}

impl Render for EditShortcutWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .min_h_0()
            .min_w_0()
            .justify_between()
            .child(
                v_flex()
                    .relative()
                    .w_full()
                    .h_auto()
                    .min_w_0()
                    .px_4()
                    .gap_2()
                    .flex_shrink_0()
                    .child(self.render_operation_line(cx))
                    .child(self.render_binding_content(cx))
                    .child(self.render_add_shortcut_btn(cx))
                    .on_prepaint(Self::resize_to_fit_content),
            )
            .child(Self::render_footer())
    }
}
