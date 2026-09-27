use gpui_kit::assets::IconName;
use gpui_kit::base::{h_flex, v_flex};
use gpui_kit::component::button::{Button, ButtonCustomVariant, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::list::{List, ListDelegate, ListItem, ListState};
use gpui_kit::component::separator::Separator;
use gpui_kit::component::{ActiveTheme, Icon, IndexPath};
use gpui_kit::*;
use lunaria_ui::components::custom_button::CustomButton;

use crate::launcher::view_model::recent_projects_view_model::RecentProjectsViewModel;

pub struct LaunchContent {
    search_input: Entity<InputState>,
    project_list: Entity<ListState<ProjectListDelegate>>,
}

impl LaunchContent {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("搜索项目")
                .default_value("")
        });

        let view_model = cx.new(|_| RecentProjectsViewModel::default());

        let project_list = cx.new(|cx| {
            let delegate = ProjectListDelegate {
                view_model: view_model.clone(),
                _subscription: RecentProjectsViewModel::subscribe(&view_model, window, cx),
            };

            ListState::new(delegate, window, cx)
                .searchable(false)
                .selectable(false)
        });

        view_model.update(cx, |view_model, cx| {
            view_model.load_recent(cx);
        });

        Self {
            search_input,
            project_list,
        }
    }

    fn render_header(&self) -> impl IntoElement {
        h_flex()
            .w_full()
            .justify_between()
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("最近项目"),
            )
            .child(
                div()
                    .w(px(180.))
                    .on_mouse_down_out(|_, window, cx| {
                        window.blur(cx);
                    })
                    .capture_key_down(|event, window, cx| {
                        if event.keystroke.key == "escape" {
                            window.blur(cx);
                        }
                    })
                    .child(
                        Input::new(&self.search_input)
                            .id("search-project")
                            .aria_label("search recent project")
                            .prefix(IconName::Search),
                    ),
            )
    }

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

    fn render_recent_project_list(&self, _cx: &mut App) -> impl IntoElement {
        v_flex()
            .w_full()
            .flex_1()
            .min_h_0()
            .child(
                Separator::horizontal()
                    .w_full()
                    .h(px(1.0))
                    .my_4()
                    .flex_shrink_0(),
            )
            .child(
                div()
                    .w_full()
                    .flex_1()
                    .min_h_0()
                    .overflow_hidden()
                    .child(List::new(&self.project_list).scrollbar_visible(true)),
            )
    }
}

impl Render for LaunchContent {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .min_w_0()
            .p_6()
            .child(self.render_header())
            // .child(Self::render_empty_state(cx))
            .child(self.render_recent_project_list(cx))
    }
}

struct ProjectListDelegate {
    view_model: Entity<RecentProjectsViewModel>,
    _subscription: Subscription,
}

impl ListDelegate for ProjectListDelegate {
    type Item = ListItem;

    fn items_count(&self, _section: usize, cx: &App) -> usize {
        self.view_model.read(cx).recent_projects().len()
    }

    fn render_item(
        &mut self,
        ix: IndexPath,
        _window: &mut Window,
        cx: &mut Context<component::list::ListState<Self>>,
    ) -> Option<Self::Item> {
        let view_model = self.view_model.read(cx);
        let project = view_model.recent_projects().get(ix.row)?;
        let id = SharedString::from(format!("project-item-{}", project.id()));
        let name = project.name().to_owned();
        let path = project.path().display().to_string();

        Some(
            ListItem::new(id)
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
                        .child(
                            div()
                                .truncate()
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(name),
                        )
                        .child(
                            div()
                                .text_xs()
                                .truncate()
                                .text_color(cx.theme().muted_foreground)
                                .child(path),
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
        LaunchContent::render_empty_state(cx)
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
