use crate::shortcuts::shortcut_editor::ShortcutEditor;

use gpui_kit::assets::IconName;
use gpui_kit::component::{
    group_box::GroupBoxVariant,
    setting::{SettingGroup, SettingItem, SettingPage as SettingsSection, Settings},
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
        let shortcuts = self.shortcuts.clone();

        let shortcuts_page = SettingsSection::new("快捷键")
            .icon(IconName::LayoutGrid)
            .group(SettingGroup::new().item(
                SettingItem::render(move |_, _, _| shortcuts.clone()).keywords([
                    "快捷键",
                    "键盘",
                    "shortcut",
                ]),
            ));

        let header_style = div().pt_6().style().clone();

        div().size_full().child(
            Settings::new("lunaria-settings")
                .sidebar_width(px(SETTING_SIDEBAR_WIDTH))
                .sidebar_size_range(px(160.)..px(280.))
                .header_style(&header_style)
                .with_group_variant(GroupBoxVariant::Normal)
                .page(shortcuts_page),
        )
    }
}
