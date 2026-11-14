use gpui_kit::App;
use lunaria_app::app_services::AppServices;
use lunaria_editor::editor_window::ProjectEditorWindow;
use std::env;
use std::path::PathBuf;

pub(crate) fn open_start_up_project(cx: &mut App) -> Result<(), String> {
    let path = env::var_os("LUNARIA_DEV_PROJECT")
        .map(PathBuf::from)
        .ok_or_else(|| "No project path set.".to_string())?;

    AppServices::run(
        cx,
        move |services| async move { services.project_service.open(path).await },
        |result, cx| match result {
            Ok(project) => ProjectEditorWindow::direct_open(project, cx),
            Err(error) => eprintln!("[Lunaria dev] {error}"),
        },
    );
    Ok(())
}
