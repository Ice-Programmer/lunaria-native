use gpui_kit::assets::IconName;
use gpui_kit::base::{h_flex, v_flex};
use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::component::list::{List, ListState};
use gpui_kit::component::separator::Separator;
use gpui_kit::*;
use lunaria_ui::components::custom_input::CustomInput;

use super::recent_product_list::ProjectListDelegate;
use crate::launcher::view_model::recent_projects_view_model::RecentProjectsViewModel;

pub struct LaunchContent {
    search_input: Entity<InputState>,
    project_list: Entity<ListState<ProjectListDelegate>>,
    _search_subscription: Subscription,
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

        let search_subscription =
            cx.subscribe_in(&search_input, window, |this, input, event, window, cx| {
                if !matches!(event, InputEvent::Change) {
                    return;
                }

                let query = input.read(cx).value().to_string();
                this.project_list.update(cx, |list, cx| {
                    list.set_query(&query, window, cx);
                });
            });

        view_model.update(cx, |view_model, cx| {
            view_model.load_recent(cx);
        });

        Self {
            search_input,
            project_list,
            _search_subscription: search_subscription,
        }
    }

    fn render_header(&self) -> impl IntoElement {
        h_flex()
            .w_full()
            .justify_between()
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("最近项目"),
            )
            .child(
                CustomInput::new(&self.search_input)
                    .id("search-project")
                    .aria_label("search recent project")
                    .prefix(IconName::Search)
                    .w(px(180.)),
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
            .child(self.render_recent_project_list(cx))
    }
}
