use gpui_kit::assets::IconName;
use gpui_kit::base::h_flex;
use gpui_kit::component::TITLE_BAR_HEIGHT;
use gpui_kit::{Context, IntoElement, ParentElement, Render, Styled, Window};
use lunaria_core::project::Project;
use lunaria_ui::components::title_bar_padding::TITLE_BAR_LEFT_PADDING;

pub struct EditorTitleBar {
    project: Project,
}

impl EditorTitleBar {
    pub fn new(project: Project) -> Self {
        EditorTitleBar { project }
    }

    fn render_project_menu_button(&self) -> impl IntoElement {
        h_flex()
            .px_3()
            .gap_2()
            .items_center()
            .child(IconName::Folder)
            .child(self.project.name().to_owned())
            .child(IconName::ChevronDown)
    }
}

impl Render for EditorTitleBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .h(TITLE_BAR_HEIGHT)
            .pl(TITLE_BAR_LEFT_PADDING)
            .child(self.render_project_menu_button())
    }
}
