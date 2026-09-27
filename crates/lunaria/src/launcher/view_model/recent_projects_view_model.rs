use gpui_kit::component::{WindowExt, notification::Notification};
use gpui_kit::{Context, Entity, EventEmitter, Subscription, Window};
use lunaria_core::project::Project;

use crate::app_services::AppServices;

pub enum RecentProjectsEvent {
    Started,
    Loaded,
    Failed(String),
}

#[derive(Default)]
pub struct RecentProjectsViewModel {
    loading: bool,
    recent_projects: Vec<Project>,
}

impl EventEmitter<RecentProjectsEvent> for RecentProjectsViewModel {}

impl RecentProjectsViewModel {
    pub fn subscribe<T: 'static>(
        view_model: &Entity<Self>,
        window: &mut Window,
        cx: &mut Context<T>,
    ) -> Subscription {
        cx.subscribe_in(view_model, window, |_, _, event, window, cx| {
            cx.notify();

            if let RecentProjectsEvent::Failed(error) = event {
                window.push_notification(Notification::error(error.clone()), cx);
            }
        })
    }

    pub fn is_loading(&self) -> bool {
        self.loading
    }

    pub fn recent_projects(&self) -> &[Project] {
        &self.recent_projects
    }

    pub fn load_recent(&mut self, cx: &mut Context<Self>) {
        if self.loading {
            return;
        }

        self.loading = true;
        cx.emit(RecentProjectsEvent::Started);

        AppServices::run(
            cx,
            |services| async move { services.project_service.list_recent().await },
            |this, result, cx| {
                this.loading = false;

                let event = match result {
                    Ok(projects) => {
                        this.recent_projects = projects;
                        RecentProjectsEvent::Loaded
                    }
                    Err(error) => RecentProjectsEvent::Failed(error),
                };

                cx.emit(event);
            },
        );
    }
}
