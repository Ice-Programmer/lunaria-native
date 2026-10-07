use crate::shortcuts::view_model::shortcut_view_model::ShortcutViewModel;
use gpui_kit::base::h_flex;
use gpui_kit::component::list::{ListDelegate, ListItem, ListState};
use gpui_kit::component::{ActiveTheme, IndexPath};
use gpui_kit::*;
use lunaria_ui::extensions::list_search::ListSearchExt;

const ROW_HEIGHT: f32 = 40.;
pub const EDIT_WIDTH: f32 = 32.0;

pub struct ShortcutListDelegate {
    pub view_model: Entity<ShortcutViewModel>,
    pub _subscription: Subscription,
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
                .child(
                    h_flex()
                        .w_full()
                        .gap_3()
                        .child(div().child(item.definition.title)),
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
                .p_3()
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
