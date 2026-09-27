use std::path::PathBuf;

use gpui_kit::component::{WindowExt, notification::Notification};
use gpui_kit::{Context, Entity, EventEmitter, Subscription, Window};
use lunaria_core::project::Project;

use crate::app_services::AppServices;

pub enum CreateProjectEvent {
    Started,
    Created(Project),
    Failed(String),
}

#[derive(Default)]
pub struct CreateProjectViewModel {
    creating: bool,
}

impl EventEmitter<CreateProjectEvent> for CreateProjectViewModel {}

impl CreateProjectViewModel {
    pub fn subscribe<T: 'static>(
        view_model: &Entity<Self>,
        window: &mut Window,
        cx: &mut Context<T>,
    ) -> Subscription {
        cx.subscribe_in(view_model, window, |_, _, event, window, cx| {
            cx.notify();

            let notification = match event {
                CreateProjectEvent::Started => return,
                CreateProjectEvent::Created(project) => {
                    Notification::success(format!("create {} project successfully", project.name()))
                }
                CreateProjectEvent::Failed(error) => Notification::error(error.clone()),
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

        self.creating = true;
        cx.emit(CreateProjectEvent::Started);

        AppServices::run(
            cx,
            move |services| async move {
                services
                    .project_service
                    .create(name, parent_directory)
                    .await
            },
            |this, result, cx| {
                this.creating = false;

                let event = match result {
                    Ok(project) => CreateProjectEvent::Created(project),
                    Err(error) => CreateProjectEvent::Failed(error),
                };

                cx.emit(event);
            },
        );
    }
}
