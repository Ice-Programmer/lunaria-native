use std::path::Path;

use crate::project::error::ProjectError;

const MAX_PROJECT_NAME_LENGTH: usize = 50;

pub struct ProjectValidator;

impl ProjectValidator {
    pub fn validate_name(name: &str) -> Result<(), ProjectError> {
        if name.is_empty() {
            return Err(ProjectError::NameRequired);
        }

        if name.chars().count() > MAX_PROJECT_NAME_LENGTH {
            return Err(ProjectError::NameTooLong {
                max: MAX_PROJECT_NAME_LENGTH,
            });
        }

        if matches!(name, "." | "..")
            || name.contains(['/', '\\'])
            || name.chars().any(char::is_control)
        {
            return Err(ProjectError::InvalidName);
        }

        Ok(())
    }

    pub fn validate_path(path: &Path) -> Result<(), ProjectError> {
        if path.as_os_str().is_empty() {
            return Err(ProjectError::PathRequired);
        }

        Ok(())
    }
}
