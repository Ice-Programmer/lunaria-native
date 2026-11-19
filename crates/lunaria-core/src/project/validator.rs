use std::path::Path;

use anyhow::{Result, bail};

const MAX_PROJECT_NAME_LENGTH: usize = 50;

pub struct ProjectValidator;

impl ProjectValidator {
    pub fn validate_name(name: &str) -> Result<()> {
        if name.is_empty() {
            bail!("project name is required");
        }

        if name.chars().count() > MAX_PROJECT_NAME_LENGTH {
            bail!("project name must not exceed {MAX_PROJECT_NAME_LENGTH} characters");
        }

        if matches!(name, "." | "..")
            || name.contains(['/', '\\'])
            || name.chars().any(char::is_control)
        {
            bail!("project name contains characters that cannot be used in a directory name");
        }

        Ok(())
    }

    pub fn validate_path(path: &Path) -> Result<()> {
        if path.as_os_str().is_empty() {
            bail!("project path is required");
        }

        Ok(())
    }
}
