#[cfg(debug_assertions)]
mod dev;
pub mod init;
mod launcher;
mod platform;

use crate::init::router::init_app_router;
use crate::launcher::launcher_page::LaunchPage;
use gpui_kit::assets::AllAssets;
use gpui_kit::*;
use lunaria_app::app_services::AppServices;
use lunaria_app::shortcuts::bindings::init_bindings;
use lunaria_app::window_root::create_window_root;
use lunaria_database::DatabaseManager;
use lunaria_settings::settings_window;
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

    let app_services = AppServices::new(databases, Handle::current()).await;
    let shortcut_items = app_services
        .shortcut_service
        .list()
        .await
        .expect("Failed to load shortcut list");

    let app = application().with_assets(AllAssets);

    app.run(move |cx| {
        init(cx);

        platform::set_application_icon();

        // init theme
        ThemeManager::init(cx).expect("Failed to initialize Lunaria themes");

        cx.set_global(app_services);

        // init global shortcut listener
        init_bindings(&shortcut_items, settings_window::open_settings, cx);

        init_app_router(cx);

        let window_options = LaunchPage::window_options(cx);

        cx.spawn(async move |cx| {
            cx.open_window(window_options, |window, cx| {
                let view = cx.new(|view_cx| LaunchPage::new(window, view_cx));

                create_window_root(view, window, cx)
            })
            .expect("Failed to open window");

            #[cfg(debug_assertions)]
            cx.update(dev::apply_dev_startup_router);
        })
        .detach();
    });
}
