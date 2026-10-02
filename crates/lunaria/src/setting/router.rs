use crate::setting::content::shortcut_content::ShortcutContent;
use gpui_kit::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SettingRoute {
    #[default]
    Shortcut,
}

impl SettingRoute {
    pub fn build(self, window: &mut Window, cx: &mut App) -> AnyView {
        match self {
            SettingRoute::Shortcut => cx.new(|cx| ShortcutContent::new(window, cx)).into(),
        }
    }
}

#[derive(Default)]
pub struct SettingRouter {
    route: SettingRoute,
}

impl Global for SettingRouter {}

impl SettingRouter {
    pub fn current(cx: &App) -> SettingRoute {
        cx.global::<Self>().route
    }

    pub fn navigate(route: SettingRoute, cx: &mut App) {
        if Self::current(cx) == route {
            return;
        }

        cx.global_mut::<Self>().route = route;
    }
}
