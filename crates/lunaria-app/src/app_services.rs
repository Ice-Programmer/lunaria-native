use std::{fmt::Display, future::Future, sync::Arc};

use gpui_kit::{App, Global};
use lunaria_core::project::ProjectService;
use lunaria_core::settings::language::service::LanguageService;
use lunaria_core::settings::shortcuts::service::ShortcutService;
use lunaria_database::DatabaseManager;
use lunaria_database::project::Repository as ProjectRepository;
use lunaria_database::settings::repository::language::Repository as LanguageRepository;
use lunaria_database::settings::repository::shortcuts::Repository as ShortcutRepository;
use lunaria_di::{Backend, BuildService, RepositoryResolver, SourceKind, resolve_repository};
use tokio::runtime::Handle;

#[derive(Clone)]
pub struct AppServices {
    pub project_service: Arc<ProjectService>,
    pub shortcut_service: Arc<ShortcutService>,
    pub language_service: Arc<LanguageService>,
    pub runtime: Handle,
    backend: Backend,
    source_kinds: SourceKind,
}

impl Global for AppServices {}

impl RepositoryResolver for AppServices {
    fn current_repository<T: 'static>(&self) -> anyhow::Result<T> {
        resolve_repository(self.source_kinds, &self.backend)
    }
}

impl AppServices {
    pub fn service<T: BuildService<Self>>(&self) -> anyhow::Result<T> {
        T::build(self)
    }
}

impl AppServices {
    pub async fn new(databases: Arc<DatabaseManager>, runtime: Handle) -> Self {
        let project_service = Arc::new(ProjectService::new(Arc::new(ProjectRepository::new(
            databases.clone(),
        ))));
        let shortcut_service =
            ShortcutService::new(ShortcutRepository::new(databases.clone())).await;

        let language_service =
            LanguageService::new(LanguageRepository::new(databases.clone())).await;

        Self {
            project_service,
            shortcut_service,
            language_service,
            runtime,
            backend: databases,
            // currently fixed Local
            source_kinds: SourceKind::Local,
        }
    }

    pub fn run<R, E, F, O>(
        cx: &mut App,
        operation: impl FnOnce(Self) -> F,
        on_result: impl FnOnce(Result<R, String>, &mut App) -> O + 'static,
    ) where
        R: Send + 'static,
        E: Display + Send + 'static,
        F: Future<Output = Result<R, E>> + Send + 'static,
    {
        let services = cx.global::<Self>().clone();
        let task = services.runtime.clone().spawn(operation(services));

        cx.spawn(async move |cx| {
            let result = task
                .await
                .map_err(|error| format!("Task execution failed: {error}"))
                .and_then(|result| result.map_err(|error| format!("{error:#}")));

            let _ = cx.update(move |cx| on_result(result, cx));
        })
        .detach();
    }
}
