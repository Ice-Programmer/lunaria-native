use gpui_kit::Global;
use lunaria_core::project::ProjectService;
use lunaria_database::DatabaseManager;
use std::sync::Arc;
use tokio::runtime::Handle;

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
}
