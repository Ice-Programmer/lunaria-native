use crate::setting::router::{SettingRoute, SettingRouter};
use gpui_kit::assets::IconName;
use gpui_kit::component::input::InputState;
use gpui_kit::component::sidebar::{Sidebar, SidebarMenu, SidebarMenuItem};
use gpui_kit::*;
use lunaria_ui::components::custom_input::CustomInput;

pub struct SettingSidebar {
    search_input: Entity<InputState>,
}

impl SettingSidebar {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("搜索设置")
                .default_value("")
        });

        Self { search_input }
    }

    fn render_search_input(&self) -> CustomInput {
        CustomInput::new(&self.search_input)
            .id("search-setting")
            .aria_label("search settings")
            .prefix(IconName::Search)
            .mt_6()
    }
}

impl Render for SettingSidebar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let current = SettingRouter::current(cx);

        Sidebar::new("setting-sidebar")
            .w_full()
            .h_full()
            .collapsible(false)
            .header(div().w_full().pb_3().child(self.render_search_input()))
            .child(
                SidebarMenu::new().child(
                    SidebarMenuItem::new("快捷键")
                        .min_h(px(36.))
                        .cursor_pointer()
                        .icon(IconName::LayoutGrid)
                        .active(current == SettingRoute::Shortcut)
                        .on_click(|_, _, cx| {
                            SettingRouter::navigate(SettingRoute::Shortcut, cx);
                        }),
                ),
            )
    }
}
