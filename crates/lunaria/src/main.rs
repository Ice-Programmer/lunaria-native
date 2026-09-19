mod launcher;
mod platform;

use gpui_kit::assets::AllAssets;
use gpui_kit::component::button::*;
use gpui_kit::component::*;
use gpui_kit::*;
use lunaria_editor::ThemeManager;

use crate::launcher::launcher_page::LaunchPage;

pub struct HelloLunaria;

impl Render for HelloLunaria {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child("Hello, Lunaria!")
            .child(
                Button::new("ok")
                    .primary()
                    .label("Change Theme!")
                    .on_click(|_, _, cx| {
                        if let Err(err) = ThemeManager::toggle(cx) {
                            eprintln!("Failed to toggle theme: {err}");
                        }
                    }),
            )
    }
}

fn main() {
    let app = gpui_kit::application().with_assets(AllAssets);

    app.run(move |cx| {
        gpui_kit::init(cx);

        platform::set_application_icon();

        // init theme
        ThemeManager::init(cx).expect("Failed to initialize Lunaria themes");

        let window_options = LaunchPage::window_options(cx);

        cx.spawn(async move |cx| {
            cx.open_window(window_options, |window, cx| {
                let view = cx.new(|_| LaunchPage::new());

                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}
