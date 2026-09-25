mod launcher;
mod platform;

use gpui_kit::assets::AllAssets;
use gpui_kit::component::*;
use gpui_kit::*;
use lunaria_database::DatabaseManager;
use lunaria_editor::ThemeManager;
use std::path::PathBuf;
use std::sync::Arc;

use crate::launcher::launcher_page::LaunchPage;

pub struct AppServices {
    pub databases: Arc<DatabaseManager>,
}

impl Global for AppServices {}

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

    let app = gpui_kit::application().with_assets(AllAssets);

    app.run(move |cx| {
        gpui_kit::init(cx);

        platform::set_application_icon();

        // init theme
        ThemeManager::init(cx).expect("Failed to initialize Lunaria themes");

        cx.set_global(AppServices {
            databases: databases.clone(),
        });

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
