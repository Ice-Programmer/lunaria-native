use gpui_kit::base::input::InputState;
use gpui_kit::base::v_flex;
use gpui_kit::component::ThemeStyled;
use gpui_kit::component::form::{Field, Form};
use gpui_kit::component::input::Input;
use gpui_kit::{div, px, AppContext, Axis, Context, Entity, FontWeight, IntoElement, ParentElement, Render, Styled, Window};
use lunaria_ui::extensions::focus::FocusExt;

pub struct CreateProjectForm {
    project_name_input: Entity<InputState>,
    product_location_input: Entity<InputState>,
}

impl CreateProjectForm {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let project_name_input = cx.new(|cx| InputState::new(window, cx));
        let product_location_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("选择项目的父目录")
                .default_value(
                    dirs::document_dir()
                        .or_else(dirs::home_dir)
                        .and_then(|path| path.into_os_string().into_string().ok())
                        .unwrap_or_default(),
                )
        });

        Self {
            project_name_input,
            product_location_input,
        }
    }

    fn render_header_title(&self) -> impl IntoElement {
        div()
            .text_2xl()
            .font_weight(FontWeight::SEMIBOLD)
            .child("新建项目")
    }

    fn render_form(&self) -> impl IntoElement {
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
                        .child(Input::new(&self.product_location_input)),
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
            .child(self.render_form())
    }
}
