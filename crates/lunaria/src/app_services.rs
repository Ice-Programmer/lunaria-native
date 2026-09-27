use std::{fmt::Display, future::Future, sync::Arc};

use gpui_kit::{Context, Global};
use lunaria_core::project::ProjectService;
use lunaria_database::DatabaseManager;
use tokio::runtime::Handle;

#[derive(Clone)]
pub struct AppServices {
    pub databases: Arc<DatabaseManager>,
    pub project_service: Arc<ProjectService>,
    pub runtime: Handle,
}

impl Global for AppServices {}

impl AppServices {
    pub fn new(databases: Arc<DatabaseManager>, runtime: Handle) -> Self {
        let repository = Arc::new(lunaria_database::project::Repository::new(
            databases.clone(),
        ));

        let project_service = Arc::new(ProjectService::new(repository));

        Self {
            databases,
            project_service,
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
