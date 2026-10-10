use crate::shortcuts::components::shortcut_list::{EDIT_WIDTH, ShortcutListDelegate};
use crate::shortcuts::view_model::shortcut_view_model::{ShortcutEvent, ShortcutViewModel};
use gpui_kit::assets::IconName;
use gpui_kit::base::{Disableable, Selectable, h_flex, v_flex};
use gpui_kit::component::ActiveTheme;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::component::list::{List, ListState};
use gpui_kit::component::{WindowExt, notification::Notification};
use gpui_kit::*;
use lunaria_ui::components::custom_input::CustomInput;
use lunaria_ui::extensions::input_18n::InputI18nExt;
use lunaria_ui::i18n::keys;
use lunaria_ui::i18n_text;

pub struct ShortcutEditor {
    search_input: Entity<InputState>,
    shortcut_list: Entity<ListState<ShortcutListDelegate>>,
    view_model: Entity<ShortcutViewModel>,
    _view_model_subscription: Subscription,
    _search_subscription: Subscription,
}

impl ShortcutEditor {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder_i18n(
                    keys::setting::shortcut::SEARCH_LABEL,
                    "搜索快捷键",
                    window,
                    cx,
                )
                .default_value("")
        });

        let view_model = cx.new(|_| ShortcutViewModel::new());

        let shortcut_list = cx.new(|cx| {
            let delegate = ShortcutListDelegate {
                view_model: view_model.clone(),
                _subscription: ShortcutViewModel::subscribe(&view_model, window, cx),
            };

            ListState::new(delegate, window, cx)
                .searchable(false)
                .selectable(false)
        });

        let view_model_subscription =
            cx.subscribe_in(&view_model, window, |_, _, event, window, cx| {
                cx.notify();

                if let ShortcutEvent::Failed(error) = event {
                    window.push_notification(Notification::error(error.clone()), cx);
                }
            });

        let search_subscription =
            cx.subscribe_in(&search_input, window, |this, input, event, window, cx| {
                if !matches!(event, InputEvent::Change) {
                    return;
                }

                let query = input.read(cx).value().to_string();
                this.shortcut_list.update(cx, |list, cx| {
                    list.set_query(&query, window, cx);
                });
            });

        view_model.update(cx, |view_model, cx| {
            view_model.load_shortcut_list(cx);
        });

        Self {
            search_input,
            shortcut_list,
            view_model,
            _view_model_subscription: view_model_subscription,
            _search_subscription: search_subscription,
        }
    }

    pub fn render_header(&self, cx: &mut Context<Self>) -> Div {
        let (total_count, modified_count, modified_only) = {
            let view_model = self.view_model.read(cx);
            (
                view_model.total_count(),
                view_model.modified_count(),
                view_model.modified_only(),
            )
        };

        h_flex()
            .gap_2()
            .child(
                CustomInput::new(&self.search_input)
                    .id("search-shortcuts")
                    .aria_label("search shortcut settings")
                    .prefix(IconName::Search)
                    .w(relative(0.3)),
            )
            .child(Button::new("search-shortcuts-commend").label(i18n_text!(
                keys::setting::shortcut::SEARCH_KEYCAPS,
                "按键检索"
            )))
            .child(
                Button::new("shortcut-filter-all")
                    .ghost()
                    .label(i18n_text!(keys::common::ALL, "全部"))
                    .selected(!modified_only)
                    .child(
                        div()
                            .text_color(cx.theme().muted_foreground)
                            .child(total_count.to_string()),
                    )
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.view_model.update(cx, |view_model, cx| {
                            view_model.set_modified_only(false, cx);
                        });
                    })),
            )
            .child(
                Button::new("shortcut-filter-modify")
                    .ghost()
                    .label(i18n_text!(keys::common::MODIFIED, "已修改"))
                    .selected(modified_only)
                    .child(
                        div()
                            .text_color(cx.theme().muted_foreground)
                            .child(modified_count.to_string()),
                    )
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.view_model.update(cx, |view_model, cx| {
                            view_model.set_modified_only(true, cx);
                        });
                    })),
            )
            .child(div().flex_1())
            .child(
                Button::new("reset-shortcut-settings")
                    .ghost()
                    .label(i18n_text!(keys::setting::RESET_ALL, "恢复全部默认"))
                    .icon(IconName::Undo2)
                    .disabled(true),
            )
    }

    fn render_column_title(&self, cx: &mut Context<Self>) -> Div {
        h_flex()
            .w_full()
            .h(px(40.))
            .px_4()
            .border_t_1()
            .border_b_1()
            .border_color(cx.theme().border)
            .text_sm()
            .text_color(cx.theme().muted_foreground)
            .child(
                div()
                    .w(relative(0.35))
                    .flex_shrink_0()
                    .child(i18n_text!(keys::common::OPERATION, "操作")),
            )
            .child(
                h_flex()
                    .flex_1()
                    .min_w_0()
                    .gap_3()
                    .child(i18n_text!(keys::setting::shortcut::LABEL, "快捷键"))
                    .child("macOS"),
            )
            .child(div().w(px(EDIT_WIDTH)).flex_shrink_0())
    }

    fn render_shortcut_list(&self, cx: &mut Context<Self>) -> Div {
        v_flex()
            .w_full()
            .flex_1()
            .min_h_0()
            .child(self.render_column_title(cx).flex_shrink_0())
            .child(
                List::new(&self.shortcut_list)
                    .w_full()
                    .flex_1()
                    .min_h_0()
                    .scrollbar_visible(true),
            )
    }
}

impl Render for ShortcutEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .gap_4()
            .child(self.render_header(cx).flex_shrink_0())
            .child(self.render_shortcut_list(cx))
    }
}
