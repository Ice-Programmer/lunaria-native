use crate::shortcuts::view_model::shortcut_view_model::ShortcutViewModel;
use gpui_kit::assets::IconName;
use gpui_kit::base::h_flex;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::kbd::Kbd;
use gpui_kit::component::list::{ListDelegate, ListItem, ListState};
use gpui_kit::component::{ActiveTheme, IndexPath};
use gpui_kit::*;
use lunaria_app::shortcuts::to_keystroke;
use lunaria_core::settings::shortcuts::model::Shortcut;
use lunaria_ui::extensions::list_search::ListSearchExt;

const ROW_HEIGHT: f32 = 40.;
pub const EDIT_WIDTH: f32 = 32.0;

pub struct ShortcutListDelegate {
    pub view_model: Entity<ShortcutViewModel>,
    pub _subscription: Subscription,
}

impl ShortcutListDelegate {
    pub fn render_keycap_v1(shortcuts: Option<Vec<Shortcut>>) -> impl IntoElement {
        let shortcuts = shortcuts.unwrap_or_default();
        let mut keycaps = h_flex().gap_2();

        if shortcuts.is_empty() {
            return keycaps.child("未设置");
        }

        for (index, shortcut) in shortcuts.iter().enumerate() {
            if index > 0 {
                keycaps = keycaps.child(div().child("或"));
            }

            keycaps = keycaps.child(Kbd::new(to_keystroke(shortcut)).flex_shrink_0());
        }

        keycaps
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

    pub fn render_keycap(shortcuts: Option<Vec<Shortcut>>, cx: &App) -> Div {
        let shortcuts = shortcuts.unwrap_or_default();

        if shortcuts.is_empty() {
            return div()
                .text_color(cx.theme().muted_foreground)
                .child("未设置");
        }

        let mut group = h_flex().gap_1();
        for (index, shortcut) in shortcuts.iter().enumerate() {
            if index > 0 {
                group = group.child(div().text_color(cx.theme().muted_foreground).child("或"))
            }

            for key in shortcut.key_parts() {
                group = group.child(Self::render_single_keycap(key, cx));
            }
        }

        group
    }
}

impl ListDelegate for ShortcutListDelegate {
    type Item = ListItem;

    fn perform_search(
        &mut self,
        query: &str,
        _window: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) -> Task<()> {
        self.view_model
            .perform_search(query, cx, ShortcutViewModel::set_query)
    }

    fn sections_count(&self, cx: &App) -> usize {
        self.view_model.read(cx).categories().len().max(1)
    }

    fn items_count(&self, section: usize, cx: &App) -> usize {
        let view_model = self.view_model.read(cx);
        view_model
            .categories()
            .get(section)
            .copied()
            .map_or(0, |category| view_model.filtered_items(category).count())
    }

    fn render_item(
        &mut self,
        ix: IndexPath,
        _window: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) -> Option<Self::Item> {
        let view_model = self.view_model.read(cx);
        let category = view_model.categories().get(ix.section).copied()?;
        let item = view_model.filtered_items(category).nth(ix.row)?;
        let action = item.definition.action;

        Some(
            ListItem::new(SharedString::from(format!("shortcut-{action:?}")))
                .w_full()
                .h(px(ROW_HEIGHT))
                .border_b_1()
                .border_color(cx.theme().border)
                .px_4()
                .text_sm()
                .child(
                    h_flex()
                        .w_full()
                        .child(
                            div()
                                .w(relative(0.35))
                                .min_w_0()
                                .flex_shrink_0()
                                .overflow_hidden()
                                .child(item.definition.title),
                        )
                        .child(
                            div()
                                .flex_1()
                                .overflow_hidden()
                                .min_w_0()
                                .child(Self::render_keycap(item.custom.clone(), cx)),
                        )
                        .child(
                            Button::new(SharedString::from(format!(
                                "edit-{}",
                                item.definition.title
                            )))
                            .text_color(cx.theme().muted_foreground)
                            .ghost()
                            .icon(IconName::Pencil),
                        ),
                ),
        )
    }

    fn render_section_header(
        &mut self,
        section: usize,
        _window: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) -> Option<impl IntoElement> {
        let category = self
            .view_model
            .read(cx)
            .categories()
            .get(section)
            .copied()?;

        Some(
            h_flex()
                .gap_3()
                .px_4()
                .pt_2()
                .items_center()
                .text_color(cx.theme().muted_foreground)
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(category.label()),
                )
                .child(div().text_xs().child(category.description())),
        )
    }

    fn loading(&self, cx: &App) -> bool {
        self.view_model.read(cx).is_loading()
    }

    fn set_selected_index(
        &mut self,
        _ix: Option<IndexPath>,
        _window: &mut Window,
        _cx: &mut Context<ListState<Self>>,
    ) {
    }
}
