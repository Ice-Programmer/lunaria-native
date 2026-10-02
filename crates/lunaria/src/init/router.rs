use crate::launcher::router::LauncherRouter;
use crate::setting::router::SettingRouter;
use gpui_kit::App;

pub fn init_app_router(cx: &mut App) {
    cx.set_global(LauncherRouter::default());
    cx.set_global(SettingRouter::default());
}
