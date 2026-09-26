use gpui_kit::base::input::{InputEvent, InputState};
use gpui_kit::base::{Disableable, h_flex, v_flex};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::form::{Field, Form};
use gpui_kit::component::input::Input;
use gpui_kit::component::{ActiveTheme, Sizable};
use gpui_kit::{
    App, AppContext, Axis, Context, Entity, FontWeight, IntoElement, ParentElement, Render, Styled,
    Subscription, Window, div, px,
};
use lunaria_ui::components::custom_button::CustomButton;
use lunaria_ui::extensions::focus::FocusExt;
use lunaria_ui::extensions::input_validator::Validator;
use lunaria_ui::extensions::subscribe_input::subscribe_input;
use std::path::PathBuf;

pub struct CreateProjectForm {
    project_name_input: Entity<InputState>,
    project_directory_input: Entity<InputState>,
    _subscriptions: Vec<Subscription>,
}

impl CreateProjectForm {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let project_name_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("untitled")
                .validate(
                    Validator::new()
                        .max_len(50)
                        .no_control_chars()
                        .forbid_chars(r#"\/:*?"<>|"#)
                        .not_in([".", ".."])
                        .build(),
                )
        });

        let project_directory_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("选择项目的父目录")
                .default_value(
                    dirs::document_dir()
                        .or_else(dirs::home_dir)
                        .and_then(|path| path.into_os_string().into_string().ok())
                        .unwrap_or_default(),
                )
        });

        let subscriptions = vec![
            subscribe_input(&project_name_input, cx),
            subscribe_input(&project_directory_input, cx),
        ];

        Self {
            project_name_input,
            project_directory_input,
            _subscriptions: subscriptions,
        }
    }

    fn fetch_project_path(&self, cx: &App) -> Option<PathBuf> {
        let name_value = self.project_name_input.read(cx).value();
        let project_name = name_value.trim();

        let directory_value = self.project_directory_input.read(cx).value();
        let project_directory = directory_value.trim();

        let parent = if project_directory == "~" {
            dirs::home_dir()?
        } else if let Some(relative) = project_directory.strip_prefix("~/") {
            dirs::home_dir()?.join(relative)
        } else {
            PathBuf::from(project_directory)
        };

        if !parent.is_absolute() || project_directory.chars().any(char::is_control) {
            return None;
        }

        Some(parent.join(project_name))
    }

    fn render_header_title(&self) -> impl IntoElement {
        div()
            .text_2xl()
            .font_weight(FontWeight::SEMIBOLD)
            .child("新建项目")
    }

    fn render_form(&self, cx: &Context<Self>) -> impl IntoElement {
        let project_path = self.fetch_project_path(cx);
        let project_name = self.project_name_input.read(cx).value();
        let can_create = project_path.is_some() && !project_name.is_empty();
        let preview = project_path
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_default();

        div().w_full().blur_on_mouse_down_out().child(
            Form::new()
                .label_layout(Axis::Horizontal)
                .label_width(px(80.))
                .child(
                    Field::new()
                        .label("项目名称：")
                        .child(Input::new(&self.project_name_input)),
                )
                .child(
                    Field::new()
                        .label("项目位置：")
                        .child(Input::new(&self.project_directory_input)),
                )
                .child(
                    Field::new().label("完整路径：").child(
                        div()
                            .w_full()
                            .min_w_0()
                            .py_1()
                            .text_sm()
                            .whitespace_normal()
                            .text_color(cx.theme().muted_foreground)
                            .child(preview),
                    ),
                )
                .footer(
                    h_flex()
                        .gap_3()
                        .child(CustomButton::new("cancel").label("取消").text_base())
                        .child(
                            CustomButton::new("save")
                                .label("创建项目")
                                .disabled(!can_create)
                                .primary()
                                .text_base(),
                        ),
                ),
        )
    }
}

impl Render for CreateProjectForm {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .min_w_0()
            .p_6()
            .gap_6()
            .child(self.render_header_title())
            .child(self.render_form(cx))
    }
}
