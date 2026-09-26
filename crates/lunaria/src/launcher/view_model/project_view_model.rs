use std::path::PathBuf;

use gpui_kit::component::{WindowExt, notification::Notification};
use gpui_kit::{Context, Entity, EventEmitter, Subscription, Window};
use lunaria_core::project::Project;

use crate::app_services::AppServices;

pub enum ProjectEvent {
    CreateStarted,
    Created(Project),
    CreateFailed(String),
}

#[derive(Default)]
pub struct ProjectViewModel {
    creating: bool,
}

impl EventEmitter<ProjectEvent> for ProjectViewModel {}

impl ProjectViewModel {
    pub fn subscribe<T: 'static>(
        view_model: &Entity<Self>,
        window: &mut Window,
        cx: &mut Context<T>,
    ) -> Subscription {
        cx.subscribe_in(view_model, window, |_, _, event, window, cx| {
            // refresh ui
            cx.notify();

            let notification = match event {
                ProjectEvent::CreateStarted => return,
                ProjectEvent::Created(project) => {
                    Notification::success(format!("create {} project successfully", project.name()))
                }
                ProjectEvent::CreateFailed(error) => Notification::error(error.clone()),
            };
            window.push_notification(notification, cx);
        })
    }

    pub fn is_creating(&self) -> bool {
        self.creating
    }

    pub fn create_project(
        &mut self,
        name: String,
        parent_directory: PathBuf,
        cx: &mut Context<Self>,
    ) {
        if self.creating {
            return;
        }

        let services = cx.global::<AppServices>();
        let service = services.project_service.clone();
        let runtime = services.runtime.clone();

        self.creating = true;
        cx.emit(ProjectEvent::CreateStarted);

        let task = runtime.spawn(async move { service.create(name, parent_directory).await });

        cx.spawn(async move |this, cx| {
            let result = task.await;
            let _ = this.update(cx, |this, cx| {
                this.creating = false;

                let event = match result {
                    Ok(Ok(project)) => ProjectEvent::Created(project),
                    Ok(Err(error)) => ProjectEvent::CreateFailed(error.to_string()),
                    Err(error) => {
                        ProjectEvent::CreateFailed(format!("create project error：{error}"))
                    }
                };
                cx.emit(event);
            });
        })
        .detach();
    }
}
