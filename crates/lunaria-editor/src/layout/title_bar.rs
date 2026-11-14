use gpui_kit::assets::IconName;
use gpui_kit::base::h_flex;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::menu::{DropdownMenu, PopupMenuItem};
use gpui_kit::component::{ActiveTheme, Icon, TITLE_BAR_HEIGHT};
use gpui_kit::{Context, IntoElement, ParentElement, Render, Styled, Window, px};
use lunaria_app::setting::shortcuts::bindings::OpenSettings;
use lunaria_core::project::Project;
use lunaria_ui::components::title_bar_padding::TITLE_BAR_LEFT_PADDING;
use lunaria_ui::i18n::keys;
use lunaria_ui::i18n_text;

pub struct EditorTitleBar {
    project: Project,
}

impl EditorTitleBar {
    pub fn new(project: Project) -> Self {
        EditorTitleBar { project }
    }

    fn render_project_menu_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let project_path = self.project.path().clone();

        Button::new("project_menu_button")
            .ghost()
            .child(
                h_flex()
                    .gap_2()
                    .text_color(cx.theme().muted_foreground)
                    .child(Icon::new(IconName::Folder))
                    .child(self.project.name().to_owned())
                    .child(Icon::new(IconName::ChevronDown)),
            )
            .dropdown_menu(move |menu, _, _| {
                let project_path = project_path.clone();

                menu.min_w(px(200.0)).item(
                    PopupMenuItem::new(i18n_text!(
                        keys::launcher::project_action::OPEN_IN_FINDER,
                        "Finder 中打开"
                    ))
                    .icon(IconName::FolderOpen)
                    .on_click(move |_, _, cx| {
                        cx.open_with_system(project_path.as_path());
                    }),
                )
            })
    }

    fn render_title_bar_action(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .pr_3()
            .text_color(cx.theme().muted_foreground)
            .child(
                Button::new("setting-btn")
                    .icon(IconName::Settings)
                    .ghost()
                    .on_click(move |_, window, cx| {
                        window.dispatch_action(Box::new(OpenSettings), cx);
                    }),
            )
    }
}

impl Render for EditorTitleBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .justify_between()
            .h(TITLE_BAR_HEIGHT)
            .pl(TITLE_BAR_LEFT_PADDING)
            .child(self.render_project_menu_button(cx))
            .child(self.render_title_bar_action(cx))
    }
}
