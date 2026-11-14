use gpui_kit::component::WindowExt;
use gpui_kit::{Context, Window};
use lunaria_app::app_services::AppServices;
use lunaria_editor::editor_window::ProjectEditorWindow;
use lunaria_ui::components::custom_notification::Notification;
use std::path::PathBuf;

#[derive(Default)]
pub struct OpenProjectViewModel {
    opening: bool,
}

impl OpenProjectViewModel {
    pub fn is_opening(&self) -> bool {
        self.opening
    }

    pub fn set_opening(&mut self, opening: bool, cx: &mut Context<Self>) {
        self.opening = opening;
        cx.notify();
    }

    pub fn open_project(&mut self, path: PathBuf, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_opening() {
            return;
        }

        self.set_opening(true, cx);

        let window_handle = window.window_handle();

        let this = cx.weak_entity();

        AppServices::run(
            cx,
            move |services| async move { services.project_service.open(path).await },
            move |result, cx| {
                this.update(cx, |this, cx| {
                    this.set_opening(false, cx);

                    window_handle.update(cx, |_, window, cx| match result {
                        Ok(project) => {
                            ProjectEditorWindow::open(project, window, cx);
                        }
                        Err(error) => {
                            window.push_notification(Notification::error(error), cx);
                        }
                    })
                })
            },
        );
    }
}
