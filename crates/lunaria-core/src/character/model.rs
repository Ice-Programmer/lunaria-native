use std::path::PathBuf;

pub struct Character {
    pub name: String,
    pub code: String,
    pub description: String,
    pub avatar_path: Option<PathBuf>,
}
