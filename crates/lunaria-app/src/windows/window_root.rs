use gpui_fps::fps_monitor;
use gpui_kit::component::Root;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;
use crate::setting::language::language_switcher::LanguageSwitcher;

struct WindowContent {
    view: AnyView,
    show_fps: bool,
    language_switcher: Entity<LanguageSwitcher>,
}

pub fn create_window_root(
    view: impl Into<AnyView>,
    window: &mut Window,
    cx: &mut App,
) -> Entity<Root> {
    let show_fps =
        std::env::var("LUNARIA_FPS").map_or(cfg!(debug_assertions), |value| value == "1");
    create_window_root_with_fps(view, show_fps, window, cx)
}

pub fn create_window_root_with_fps(
    view: impl Into<AnyView>,
    show_fps: bool,
    window: &mut Window,
    cx: &mut App,
) -> Entity<Root> {
    let language_switcher = cx.new(|_| LanguageSwitcher::new());
    let content = cx.new(|_| WindowContent {
        view: view.into(),
        show_fps,
        language_switcher,
    });
    cx.new(|cx| Root::new(content, window, cx))
}

pub fn get_language_switcher(
    window: &mut Window,
    cx: &App,
) -> Option<Entity<LanguageSwitcher>> {
    let content = window
        .root::<Root>()
        .flatten()?
        .read(cx)
        .view()
        .clone()
        .downcast::<WindowContent>()
        .ok()?;

    Some(content.read(cx).language_switcher.clone())
}

impl Render for WindowContent {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .child(self.view.clone())
            .when(self.show_fps, |layout| {
                layout.child(fps_monitor(window, cx).anchor(Anchor::BottomRight))
            })
            .child(deferred(self.language_switcher.clone()).with_priority(1))
            .children(Root::render_notification_layer(window, cx))
            .children(Root::render_dialog_layer(window, cx))
    }
}
