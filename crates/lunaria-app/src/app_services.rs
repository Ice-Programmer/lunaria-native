use std::{fmt::Display, future::Future, sync::Arc};

use gpui_kit::{Context, Global};
use lunaria_core::project::ProjectService;
use lunaria_core::settings::shortcuts::service::ShortcutService;
use lunaria_database::DatabaseManager;
use lunaria_database::project::Repository as ProjectRepository;
use lunaria_database::settings::repository::shortcuts::Repository as ShortcutRepository;
use tokio::runtime::Handle;

#[derive(Clone)]
pub struct AppServices {
    pub project_service: Arc<ProjectService>,
    pub shortcut_service: Arc<ShortcutService>,
    pub runtime: Handle,
}

impl Global for AppServices {}

impl AppServices {
    pub async fn new(databases: Arc<DatabaseManager>, runtime: Handle) -> Self {
        let project_service = Arc::new(ProjectService::new(Arc::new(ProjectRepository::new(
            databases.clone(),
        ))));
        let shortcut_service =
            ShortcutService::new(ShortcutRepository::new(databases.clone())).await;

        Self {
            project_service,
            shortcut_service,
            runtime,
        }
    }

    pub fn run<T, R, E, F>(
        cx: &mut Context<T>,
        operation: impl FnOnce(Self) -> F,
        on_result: impl FnOnce(&mut T, Result<R, String>, &mut Context<T>) + 'static,
    ) where
        T: 'static,
        R: Send + 'static,
        E: Display + Send + 'static,
        F: Future<Output = Result<R, E>> + Send + 'static,
    {
        let services = cx.global::<Self>().clone();
        let runtime = services.runtime.clone();

        let task = runtime.spawn(operation(services));

        cx.spawn(async move |this, cx| {
            let result = task
                .await
                .map_err(|error| format!("Task execution failed: {error}"))
                .and_then(|result| result.map_err(|error| error.to_string()));

            let _ = this.update(cx, |this, cx| {
                on_result(this, result, cx);
            });
        })
        .detach();
    }
}
