use gpui_kit::component::setting::SettingPage as SettingsSection;
use gpui_kit::*;

pub const SETTINGS_HEADER_HEIGHT: f32 = 50.;

pub fn render_settings_section(title: impl Into<SharedString>) -> SettingsSection {
    SettingsSection::new(title).header_style(
        &div()
            .h(px(SETTINGS_HEADER_HEIGHT))
            .py_0()
            .justify_center()
            .font_weight(FontWeight::SEMIBOLD)
            .flex_shrink_0()
            .style()
            .clone(),
    )
}
