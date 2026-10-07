use super::shortcut_editor::ShortcutEditor;
use crate::components::settings_section::{SETTINGS_HEADER_HEIGHT, render_settings_section};
use gpui_kit::assets::IconName;
use gpui_kit::component::setting::{SettingGroup, SettingItem, SettingPage as SettingsSection};
use gpui_kit::*;

pub fn build_shortcut_page(shortcuts: Entity<ShortcutEditor>) -> SettingsSection {
    render_settings_section("快捷键")
        .icon(IconName::LayoutGrid)
        .group(
            SettingGroup::new().item(SettingItem::render(move |_, window, _| {
                let height = (window.viewport_size().height
                    - px(SETTINGS_HEADER_HEIGHT)
                    - window.rem_size() * 2.)
                    .max(px(0.));

                div()
                    .w_full()
                    .h(height)
                    .overflow_hidden()
                    .child(shortcuts.clone())
            })),
        )
}
