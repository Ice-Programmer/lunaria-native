use crate::launcher::view_model::recent_projects_view_model::RecentProjectsViewModel;
use gpui_kit::assets::IconName;
use gpui_kit::base::v_flex;
use gpui_kit::component::button::{Button, ButtonCustomVariant, ButtonVariants};
use gpui_kit::component::list::{ListDelegate, ListItem, ListState};
use gpui_kit::component::{ActiveTheme, Icon, IndexPath};
use gpui_kit::*;

pub struct ProjectListDelegate {
    pub view_model: Entity<RecentProjectsViewModel>,
    pub _subscription: Subscription,
}

impl ProjectListDelegate {
    fn render_empty_state(cx: &App) -> impl IntoElement {
        v_flex()
            .flex_1()
            .items_center()
            .justify_center()
            .gap_3()
            .pb_8()
            .child(
                Icon::new(IconName::Folder)
                    .size(px(32.))
                    .text_color(cx.theme().muted_foreground),
            )
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("还没有最近项目"),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .child("新建一个故事，或打开已有的项目文件"),
            )
    }
}

impl ListDelegate for ProjectListDelegate {
    type Item = ListItem;

    fn perform_search(
        &mut self,
        query: &str,
        _window: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) -> Task<()> {
        self.view_model.update(cx, |view_model, cx| {
            view_model.set_query(query, cx);
        });
        Task::ready(())
    }

    fn items_count(&self, _section: usize, cx: &App) -> usize {
        self.view_model.read(cx).filtered_projects().count()
    }

    fn render_item(
        &mut self,
        ix: IndexPath,
        _window: &mut Window,
        cx: &mut Context<component::list::ListState<Self>>,
    ) -> Option<Self::Item> {
        let project = self.view_model.read(cx).filtered_projects().nth(ix.row)?;

        Some(
            ListItem::new(SharedString::from(format!("project-item-{}", project.id())))
                .w_full()
                .gap_4()
                .rounded_lg()
                .px_4()
                .py_3()
                .cursor_pointer()
                .child(
                    v_flex()
                        .h_full()
                        .justify_between()
                        .flex_1()
                        .min_w_0()
                        .child(div().truncate().child(project.name().to_owned()))
                        .child(
                            div()
                                .text_xs()
                                .truncate()
                                .text_color(cx.theme().muted_foreground)
                                .child(project.path().display().to_string()),
                        ),
                )
                .suffix(|_, cx| {
                    Button::new("more")
                        .ghost()
                        .custom(ButtonCustomVariant::new(cx))
                        .cursor_pointer()
                        .child(Icon::new(IconName::EllipsisVertical))
                }),
        )
    }

    fn render_empty(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<component::list::ListState<Self>>,
    ) -> impl IntoElement {
        if self.view_model.read(cx).query().is_empty() {
            Self::render_empty_state(cx).into_any_element()
        } else {
            div().into_any_element()
        }
    }

    fn loading(&self, cx: &App) -> bool {
        self.view_model.read(cx).is_loading()
    }

    fn set_selected_index(
        &mut self,
        _ix: Option<IndexPath>,
        _window: &mut Window,
        _cx: &mut Context<component::list::ListState<Self>>,
    ) {
        return;
    }
}
