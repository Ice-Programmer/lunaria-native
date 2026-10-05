use crate::launcher::router::LauncherRouter;
use gpui_kit::App;

pub fn init_app_router(cx: &mut App) {
    cx.set_global(LauncherRouter::default());
}
