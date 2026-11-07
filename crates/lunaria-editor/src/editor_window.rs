use gpui_kit::*;
use lunaria_app::windows::custom_window;
use lunaria_app::windows::custom_window::WindowConfig;
use lunaria_core::project::Project;

const EDITOR_WIDTH: f32 = 1200.;
const EDITOR_HEIGHT: f32 = 800.;
const EDITOR_MIN_WIDTH: f32 = 800.;
const EDITOR_MIN_HEIGHT: f32 = 600.;

pub struct ProjectEditorWindow;

impl ProjectEditorWindow {
    pub fn open(project: &Project, launcher: &mut Window, cx: &mut App) {
        let launcher_handler = launcher.window_handle();

        custom_window::open(
            (),
            WindowConfig {
                title: Some(project.name().to_owned().into()),
                size: size(px(EDITOR_WIDTH), px(EDITOR_HEIGHT)),
                min_size: Some(size(px(EDITOR_MIN_WIDTH), px(EDITOR_MIN_HEIGHT))),
                ..Default::default()
            },
            cx,
            move |_, cx| {
                cx.defer(move |cx| {
                    let _ = launcher_handler.update(cx, |_, window, _| {
                        window.remove_window();
                    });
                });

                Self
            },
        );
    }
}

impl Render for ProjectEditorWindow {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full()
    }
}
