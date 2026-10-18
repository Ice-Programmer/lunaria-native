use gpui_kit::SharedString;
use lunaria_core::project::ProjectError;
use lunaria_ui::i18n_text;

pub(super) fn  (error: &ProjectError) -> SharedString {
    match error {
        ProjectError::NameRequired => {
            i18n_text!("launcher.create.name_required", "请输入项目名称")
        }
        ProjectError::NameTooLong { max } => i18n_text!(
            "launcher.create.name_too_long",
            "项目名称不能超过 %{max} 个字符",
            max = max
        ),
        ProjectError::InvalidName => i18n_text!(
            "launcher.create.invalid_name",
            "项目名称包含不能用于目录名称的字符"
        ),
        ProjectError::PathRequired => {
            i18n_text!("launcher.create.path_required", "请选择项目目录")
        }
        ProjectError::Storage(message) => i18n_text!(
            "launcher.create.storage_failed",
            "项目存储操作失败：%{message}",
            message = message
        ),
    }
}
