use crate::DatabaseManager;
use std::sync::Arc;

pub struct ProjectRepository {
    databases: Arc<DatabaseManager>,
}

impl ProjectRepository {
    pub fn new(databases: Arc<DatabaseManager>) -> Self {
        Self { databases }
    }
}
