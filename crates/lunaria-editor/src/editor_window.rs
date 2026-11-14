use crate::layout::editor_layout::EditorLayout;
use crate::layout::sidebar::EditorSidebar;
use crate::layout::status_bar::EditorStatusBar;
use crate::layout::title_bar::EditorTitleBar;
use crate::route::editor_route::EditorRoute;
use gpui_kit::*;
use lunaria_app::windows::custom_window;
use lunaria_app::windows::custom_window::WindowConfig;
use lunaria_core::project::Project;
use std::collections::HashMap;

const EDITOR_WIDTH: f32 = 1200.;
const EDITOR_HEIGHT: f32 = 800.;
const EDITOR_MIN_WIDTH: f32 = 800.;
const EDITOR_MIN_HEIGHT: f32 = 600.;

pub struct ProjectEditorWindow {
    title_bar: Entity<EditorTitleBar>,
    sidebar: Entity<EditorSidebar>,
    status_bar: Entity<EditorStatusBar>,
    pages: HashMap<EditorRoute, AnyView>,
}

impl ProjectEditorWindow {
    fn new(cx: &mut Context<Self>, project: Project) -> Self {
        Self {
            title_bar: cx.new(|_| EditorTitleBar::new(project)),
            sidebar: cx.new(|_| EditorSidebar::new(EditorRoute::default())),
            status_bar: cx.new(|_| EditorStatusBar),
            pages: HashMap::new(),
        }
    }

    pub fn open(project: Project, launcher: &mut Window, cx: &mut App) {
        let launcher_handler = launcher.window_handle();

        custom_window::open(
            (),
            WindowConfig {
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

                Self::new(cx, project)
            },
        );
    }

    pub fn direct_open(project: Project, cx: &mut App) {
        custom_window::open(
            (),
            WindowConfig {
                size: size(px(EDITOR_WIDTH), px(EDITOR_HEIGHT)),
                min_size: Some(size(px(EDITOR_MIN_WIDTH), px(EDITOR_MIN_HEIGHT))),
                ..Default::default()
            },
            cx,
            |_, cx| Self::new(cx, project),
        )
    }
}

impl Render for ProjectEditorWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let route = self.sidebar.read(cx).route();

        let content = self
            .pages
            .entry(route)
            .or_insert_with(|| route.build(cx))
            .clone();

        EditorLayout::new(
            self.title_bar.clone(),
            self.sidebar.clone(),
            div().size_full().items_center().child(content),
            self.status_bar.clone(),
        )
    }
}
