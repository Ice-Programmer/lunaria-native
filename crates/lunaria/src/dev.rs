use crate::launcher::router::{LauncherRoute, LauncherRouter};
use crate::setting::router::{SettingRoute, SettingRouter};
use crate::setting::setting_page::SettingPage;
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
        "setting" => {
            let route = match page {
                "shortcut" => SettingRoute::Shortcut,
                unknown => {
                    return Err(format!("Unknown setting route: {unknown}"));
                }
            };
            SettingRouter::navigate(route, cx);
            SettingPage::open(cx);

            Ok(())
        }

        unknown => Err(format!("Unknown module: {unknown}")),
    }
}
