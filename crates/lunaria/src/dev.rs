use crate::launcher::router::{LauncherRoute, LauncherRouter};
use gpui_kit::App;
use std::env;

const DEV_ROUTE_ENV: &str = "LUNARIA_DEV_ROUTE";

pub(crate) fn apply_dev_startup_router(cx: &mut App) {
    let Ok(value) = env::var(DEV_ROUTE_ENV) else {
        return;
    };

    if let Err(err) = navigate_dev_route(value.trim(), cx) {
        eprintln!("[Lunaria dev] {err}");
    }
}

fn navigate_dev_route(value: &str, cx: &mut App) -> Result<(), String> {
    let (module, page) = value
        .split_once('/')
        .ok_or_else(|| "expected 'module/page'".to_string())?;

    match module {
        "launcher" => {
            let route = match page {
                "recent" => LauncherRoute::Recent,
                "create-project" => LauncherRoute::CreateProject,
                unknown => {
                    return Err(format!("Unknown launcher route: {unknown}"));
                }
            };

            Ok(LauncherRouter::navigate(route, cx))
        }

        unknown => Err(format!("Unknown module: {unknown}")),
    }
}
