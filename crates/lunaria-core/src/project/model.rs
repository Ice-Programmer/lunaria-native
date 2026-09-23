use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Project {
    id: i64,
    name: String,
    path: PathBuf,
    created_at: i64,
    last_opened_at: i64,
}

impl Project {
    pub fn new(id: i64, name: String, path: PathBuf, created_at: i64, last_opened_at: i64) -> Self {
        Self {
            id,
            name,
            path,
            created_at,
            last_opened_at,
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn created_at(&self) -> i64 {
        self.created_at
    }

    pub fn last_opened_at(&self) -> i64 {
        self.last_opened_at
    }
}

