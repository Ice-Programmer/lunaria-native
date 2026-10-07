use crate::shortcuts::shortcut_view::shortcut_editor::ShortcutEditor;

use crate::shortcuts::shortcut_view::shortcut_page::build_shortcut_page;
use gpui_kit::component::{
    group_box::GroupBoxVariant,
    setting::Settings,
};
use gpui_kit::*;

const SETTING_SIDEBAR_WIDTH: f32 = 200.0;

pub struct SettingPage {
    shortcuts: Entity<ShortcutEditor>,
}

impl SettingPage {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            shortcuts: cx.new(|cx| ShortcutEditor::new(window, cx)),
        }
    }
}

impl Render for SettingPage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(
            Settings::new("lunaria-settings")
                .sidebar_width(px(SETTING_SIDEBAR_WIDTH))
                .header_style(&div().pt_6().style().clone())
                .with_group_variant(GroupBoxVariant::Normal)
                .page(build_shortcut_page(self.shortcuts.clone())),
        )
    }
}
