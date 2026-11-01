use crate::shortcuts::components::shortcut_recorder::ShortcutRecorder;
use crate::shortcuts::i18n::definition_i18n::ShortcutDefinitionI18nExt;
use crate::shortcuts::view_model::shortcut_view_model::ShortcutViewModel;
use gpui_kit::base::{h_flex, v_flex};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{ActiveTheme, ElementExt, IconName, Sizable, TITLE_BAR_HEIGHT};
use gpui_kit::gpui::actions;
use gpui_kit::*;
use lunaria_app::windows::custom_window::{self, WindowConfig};
use lunaria_core::settings::shortcuts::model::{Shortcut, ShortcutAction, ShortcutItem};
use lunaria_ui::i18n::keys;
use lunaria_ui::i18n_text;

actions!(shortcut_editor, [CloseShortcutEditor]);

const SHORTCUT_WINDOW_WIDTH: f32 = 260.;
const SHORTCUT_WINDOW_MIN_HEIGHT: f32 = 180.;
const SHORTCUT_ROW_HEIGHT: f32 = 32.;
const SHORTCUT_WINDOW_MAX_HEIGHT: f32 = 600.;
const SHORTCUT_FOOTER_HEIGHT: f32 = 56.;

pub struct EditShortcutWindow {
    shortcut: ShortcutItem,
    view_model: Entity<ShortcutViewModel>,
    recorders: Vec<Entity<ShortcutRecorder>>,
    last_auto_height: Option<Pixels>,
}

impl EditShortcutWindow {
    fn new(
        shortcut: ShortcutItem,
        view_model: Entity<ShortcutViewModel>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut recorders = Vec::new();

        for value in shortcut.custom.as_deref().unwrap_or_default() {
            recorders.push(cx.new(|cx| ShortcutRecorder::new(Some(value.clone()), window, cx)));
        }

        if recorders.is_empty() {
            recorders.push(cx.new(|cx| ShortcutRecorder::new(None, window, cx)));
        }

        Self {
            shortcut,
            view_model,
            recorders,
            last_auto_height: None,
        }
    }

    fn resotre_defaults(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        window.blur(cx);

        let mut recorders = self
            .shortcut
            .definition
            .defaults
            .iter()
            .map(|record| cx.new(|cx| ShortcutRecorder::new(Some(record.clone()), window, cx)))
            .collect::<Vec<_>>();

        if recorders.is_empty() {
            recorders.push(cx.new(|cx| ShortcutRecorder::new(None, window, cx)));
        }

        self.recorders = recorders;
        cx.notify()
    }

    pub fn open(shortcut: ShortcutItem, view_model: Entity<ShortcutViewModel>, cx: &mut App) {
        custom_window::open(
            shortcut.definition.action,
            WindowConfig {
                title: Some(format!("{}", shortcut.definition.title_i18n()).into()),
                size: size(px(SHORTCUT_WINDOW_WIDTH), px(SHORTCUT_WINDOW_MIN_HEIGHT)),
                need_fullscreen: false,
                ..Default::default()
            },
            cx,
            move |window, cx| Self::new(shortcut, view_model, window, cx),
        )
    }

    fn render_operation_line(&self, cx: &Context<Self>) -> Div {
        h_flex()
            .w_full()
            .min_w_0()
            .gap_3()
            .justify_between()
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .child(i18n_text!(
                        keys::setting::shortcut::edit_dialog::RECORD_HINT,
                        "点击后按下组合键"
                    )),
            )
            .child(
                Button::new("reset-shortcut")
                    .ghost()
                    .small()
                    .flex_shrink_0()
                    .text_color(cx.theme().muted_foreground)
                    .label(i18n_text!(
                        keys::setting::shortcut::edit_dialog::RESTORE_DEFAULTS,
                        "恢复默认"
                    ))
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.resotre_defaults(window, cx);
                    })),
            )
    }

    fn render_binding_content(&self, cx: &Context<Self>) -> Div {
        v_flex()
            .w_full()
            .min_w_0()
            .gap_2()
            .children(self.recorders.iter().map(|recorder| {
                let recorder_id = recorder.entity_id();
                div().w_full().child(recorder.clone()).child(
                    h_flex().absolute().top_0().right_2().bottom_0().child(
                        Button::new(("remove-shortcut", recorder_id))
                            .ghost()
                            .xsmall()
                            .icon(IconName::Close)
                            .tooltip(i18n_text!(
                                keys::setting::shortcut::edit_dialog::REMOVE_SHORTCUT,
                                "删除快捷键"
                            ))
                            .text_color(cx.theme().muted_foreground)
                            .cursor_pointer()
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.recorders
                                    .retain(|item| item.entity_id() != recorder_id);

                                cx.notify();
                            })),
                    ),
                )
            }))
    }

    fn render_add_shortcut_btn(&mut self, cx: &mut Context<Self>) -> Option<Button> {
        if self.recorders.len() >= 3 {
            return None;
        }

        Some(
            Button::new("add-shortcut")
                .icon(IconName::Plus)
                .ghost()
                .small()
                .self_start()
                .text_color(cx.theme().muted_foreground)
                .label(i18n_text!(
                    keys::setting::shortcut::edit_dialog::ADD_SHORTCUT,
                    "添加快捷键"
                ))
                .on_click(cx.listener(|this, _, window, cx| {
                    this.recorders
                        .push(cx.new(|cx| ShortcutRecorder::new(None, window, cx)));
                    cx.notify();
                })),
        )
    }

    fn resize_to_fit_content(
        &mut self,
        bounds: Bounds<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // 内容高度 + 自定义标题栏高度 + footer
        let height = (bounds.size.height + TITLE_BAR_HEIGHT + px(SHORTCUT_FOOTER_HEIGHT))
            .ceil()
            .clamp(
                px(SHORTCUT_WINDOW_MIN_HEIGHT),
                px(SHORTCUT_WINDOW_MAX_HEIGHT),
            );

        if self.last_auto_height == Some(height) {
            return;
        }

        self.last_auto_height = Some(height);

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

    fn render_footer(&self, cx: &Context<Self>) -> Div {
        h_flex()
            .w_full()
            .flex_shrink_0()
            .justify_end()
            .p_4()
            .gap_3()
            .child(
                Button::new("cancel-shortcut")
                    .label(i18n_text!(keys::common::CANCEL, "取消"))
                    .on_click(|_, window, _| window.remove_window()),
            )
            .child(
                Button::new("submit-shortcut")
                    .primary()
                    .label(i18n_text!(keys::common::SAVE, "保存"))
                    .on_click(cx.listener(|this, _, window, cx| {
                        let shortcuts: Vec<Shortcut> = this
                            .recorders
                            .iter()
                            .filter_map(|recorder| recorder.read(cx).value().cloned())
                            .collect();

                        window.blur(cx);

                        this.view_model.update(cx, |view_model, cx| {
                            view_model.save_shortcut(
                                cx,
                                this.shortcut.definition.action,
                                shortcuts,
                                window,
                            );
                        })
                    })),
            )
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
                    .on_prepaint(cx.processor(Self::resize_to_fit_content))
                    .child(self.render_operation_line(cx))
                    .child(self.render_binding_content(cx))
                    .children(self.render_add_shortcut_btn(cx)),
            )
            .child(self.render_footer(cx))
    }
}
