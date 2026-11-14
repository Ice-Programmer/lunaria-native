use gpui_kit::{App, Menu, MenuItem};
use lunaria_app::setting::shortcuts::bindings::Quit;

pub(crate) fn init_macos_config(cx: &mut App) {

    cx.set_menus([Menu::new("Lunaria").items([MenuItem::action("退出 Lunaria", Quit)])]);

    cx.activate(true);
}
