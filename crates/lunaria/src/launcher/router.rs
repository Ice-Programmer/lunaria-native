use gpui_kit::*;

use super::components::LaunchContent;
use super::components::create_project_form::CreateProjectForm;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LauncherRoute {
    #[default]
    Recent,

    CreateProject,
}

impl LauncherRoute {
    pub fn build(self, window: &mut Window, cx: &mut App) -> AnyView {
        match self {
            Self::Recent => cx.new(|cx| LaunchContent::new(window, cx)).into(),

            Self::CreateProject => cx.new(|cx| CreateProjectForm::new(window, cx)).into(),
        }
    }
}

#[derive(Default)]
pub struct LauncherRouter {
    route: LauncherRoute,
}

impl Global for LauncherRouter {}

impl LauncherRouter {
    pub fn current(cx: &App) -> LauncherRoute {
        cx.global::<Self>().route
    }

    pub fn navigate(route: LauncherRoute, cx: &mut App) {
        if Self::current(cx) == route {
            return;
        }

        cx.global_mut::<Self>().route = route;
    }
}
