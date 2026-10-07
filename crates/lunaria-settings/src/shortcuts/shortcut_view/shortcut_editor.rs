use crate::shortcuts::shortcut_view::components::shortcut_list::{
    EDIT_WIDTH, ShortcutListDelegate,
};
use crate::shortcuts::shortcut_view::shortcut_view_model::{ShortcutEvent, ShortcutViewModel};
use gpui_kit::assets::IconName;
use gpui_kit::base::{Disableable, h_flex, v_flex};
use gpui_kit::component::ActiveTheme;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::InputState;
use gpui_kit::component::list::{List, ListState};
use gpui_kit::*;
use lunaria_ui::components::custom_input::CustomInput;

pub struct ShortcutEditor {
    search_input: Entity<InputState>,
    shortcut_list: Entity<ListState<ShortcutListDelegate>>,
    _subscription: Subscription,
}

impl ShortcutEditor {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("搜索快捷键")
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

        let subscription = cx.subscribe(&view_model, |this, _, _event: &ShortcutEvent, cx| {
            this.shortcut_list.update(cx, |_, cx| cx.notify());
            cx.notify();
        });

        view_model.update(cx, |view_model, cx| {
            view_model.load_shortcut_list(cx);
        });

        Self {
            search_input,
            shortcut_list,
            _subscription: subscription,
        }
    }

    pub fn render_header(&self, cx: &mut Context<Self>) -> Div {
        h_flex()
            .gap_2()
            .child(
                CustomInput::new(&self.search_input)
                    .id("search-shortcuts")
                    .aria_label("search shortcut settings")
                    .prefix(IconName::Search)
                    .w(relative(0.3)),
            )
            .child(Button::new("search-shortcuts-commend").label("按键检索"))
            .child(
                Button::new("shortcut-filter-all")
                    .ghost()
                    .label("全部")
                    .child(div().text_color(cx.theme().muted_foreground).child("12")),
            )
            .child(
                Button::new("shortcut-filter-modify")
                    .ghost()
                    .label("已修改")
                    .child(div().text_color(cx.theme().muted_foreground).child("8")),
            )
            .child(div().flex_1())
            .child(
                Button::new("reset-shortcut-settings")
                    .ghost()
                    .label("恢复全部默认")
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
            .child(div().w(relative(0.5)).flex_shrink_0().child("操作"))
            .child(
                h_flex()
                    .flex_1()
                    .min_w_0()
                    .gap_3()
                    .child("快捷键")
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
