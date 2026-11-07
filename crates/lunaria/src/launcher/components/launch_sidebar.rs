use crate::launcher::router::{LauncherRoute, LauncherRouter};
use crate::launcher::view_model::open_project_view_model::OpenProjectViewModel;
use gpui_kit::assets::IconName;
use gpui_kit::base::{Selectable, h_flex, v_flex};
use gpui_kit::component::button::*;
use gpui_kit::component::{ActiveTheme, Icon};
use gpui_kit::*;
use lunaria_ui::components::custom_button::CustomButton;
use lunaria_ui::i18n::keys;
use lunaria_ui::i18n_text;
use rfd::AsyncFileDialog;

const BUTTON_HEIGHT: f32 = 36.;

#[derive(IntoElement)]
pub struct LaunchSidebar;

impl LaunchSidebar {
    pub fn new() -> Self {
        Self
    }

    fn render_brand(cx: &mut App) -> impl IntoElement {
        v_flex()
            .items_start()
            .gap_0()
            .child(
                div()
                    .text_3xl()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("Lunaria"),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .child(i18n_text!(keys::launcher::SUBTITLE, "视觉小说创作空间")),
            )
    }

    fn render_action(window: &mut Window, cx: &mut App) -> impl IntoElement {
        let view_model = window.use_keyed_state("launcher-open-project", cx, |_, _| {
            OpenProjectViewModel::default()
        });

        let creating = LauncherRouter::current(cx) == LauncherRoute::CreateProject;
        v_flex()
            .gap_2()
            .child(
                Button::new("create-project")
                    .mt_8()
                    .label(i18n_text!(
                        keys::launcher::main_button::CREATE_PROJECT,
                        "创建项目"
                    ))
                    .primary()
                    .w_full()
                    .selected(creating)
                    .h(px(BUTTON_HEIGHT))
                    .icon(IconName::Plus)
                    .cursor_pointer()
                    .on_click(|_, _, cx| {
                        LauncherRouter::navigate(LauncherRoute::CreateProject, cx)
                    }),
            )
            .child(
                CustomButton::new("open-project")
                    .label(i18n_text!(
                        keys::launcher::main_button::OPEN_PROJECT,
                        "打开项目"
                    ))
                    .w_full()
                    .icon(IconName::FolderOpen)
                    .h(px(BUTTON_HEIGHT))
                    .cursor_pointer()
                    .loading(view_model.read(cx).is_opening())
                    .on_click(move |_, window, cx| {
                        let view_model = view_model.clone();
                        let dialog = AsyncFileDialog::new().set_parent(&*window);

                        window
                            .spawn(cx, async move |cx| {
                                let Some(folder) = dialog.pick_folder().await else {
                                    return;
                                };

                                let path = folder.path().to_path_buf();

                                let _ = cx.update(|window, cx| {
                                    view_model.update(cx, |view_model, cx| {
                                        view_model.open_project(path, window, cx);
                                    });
                                });
                            })
                            .detach();
                    }),
            )
    }

    fn action_item(
        cx: &mut App,
        icon: IconName,
        label: impl Into<SharedString>,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> impl IntoElement {
        let label: SharedString = label.into();

        Button::new(label.clone())
            .ghost()
            .w_full()
            .h(px(32.))
            .child(
                h_flex()
                    .w_full()
                    .items_center()
                    .gap_4()
                    .child(
                        Icon::new(icon)
                            .size_5()
                            .mt(px(1.))
                            .text_color(cx.theme().muted_foreground),
                    )
                    .child(
                        div()
                            .text_base()
                            .child(label)
                            .text_color(cx.theme().muted_foreground),
                    ),
            )
            .on_click(on_click)
    }

    // todo need add action
    fn render_action_bottom(cx: &mut App) -> impl IntoElement {
        v_flex()
            .gap_2()
            .mb_4()
            .child(Self::action_item(
                cx,
                IconName::BookOpen,
                i18n_text!(keys::launcher::footer_button::GUIDE, "使用指南"),
                |_, _, _| {
                    println!("open user guide");
                },
            ))
            .child(Self::action_item(
                cx,
                IconName::MessageCircle,
                i18n_text!(keys::launcher::footer_button::FEEDBACK, "问题反馈"),
                |_, _, _| {
                    println!("open feedback");
                },
            ))
            .child(Self::action_item(
                cx,
                IconName::Github,
                "GitHub",
                |_, _, _| {
                    println!("open github");
                },
            ))
            .child(Self::action_item(
                cx,
                IconName::Info,
                "v0.1.0",
                |_, _, _| {
                    println!("open version");
                },
            ))
    }
}

impl RenderOnce for LaunchSidebar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .size_full()
            .px_6()
            .py_4()
            .border_r_1()
            .border_color(cx.theme().sidebar_border)
            .bg(cx.theme().sidebar)
            .child(Self::render_brand(cx))
            .child(Self::render_action(window, cx))
            .child(div().flex_1())
            .child(Self::render_action_bottom(cx))
    }
}
