mod app_services;
#[cfg(debug_assertions)]
mod dev;
mod launcher;
mod platform;

use crate::app_services::AppServices;
use crate::launcher::launcher_page::LaunchPage;
use crate::launcher::router::{LauncherRoute, LauncherRouter};
use gpui_kit::assets::AllAssets;
use gpui_kit::component::Root;
use gpui_kit::*;
use lunaria_database::DatabaseManager;
use lunaria_ui::ThemeManager;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::runtime::Handle;

// get Lunaria app directory
fn app_data_directory() -> PathBuf {
    dirs::data_local_dir()
        .expect("Failed to locate the operating system application data directory")
        .join("Lunaria")
}

#[tokio::main]
async fn main() {
    // get the application data directory
    let app_data_directory = app_data_directory();

    // init app database
    let databases = Arc::new(
        DatabaseManager::initialize(&app_data_directory)
            .await
            .expect("Failed to initialize Lunaria databases"),
    );

    let app_services = AppServices::new(databases, Handle::current());

    let app = application().with_assets(AllAssets);

    app.run(move |cx| {
        init(cx);

        platform::set_application_icon();

        // init theme
        ThemeManager::init(cx).expect("Failed to initialize Lunaria themes");

        cx.set_global(app_services);
        cx.set_global(LauncherRouter::default());

        #[cfg(debug_assertions)]
        dev::apply_dev_startup_router(cx);

        let window_options = LaunchPage::window_options(cx);

        cx.spawn(async move |cx| {
            cx.open_window(window_options, |window, cx| {
                let view = cx.new(|view_cx| LaunchPage::new(window, view_cx));

                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}
