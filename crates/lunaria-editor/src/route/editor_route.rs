use gpui_kit::SharedString;
use gpui_kit::assets::IconName;
use lunaria_ui::i18n::keys;
use lunaria_ui::i18n_text;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum EditorRoute {
    #[default]
    Script,
    Stage,
    Character,
}

impl EditorRoute {
    pub const ALL: [Self; 3] = [Self::Script, Self::Stage, Self::Character];

    pub fn label_and_icon(self) -> (SharedString, IconName) {
        match self {
            EditorRoute::Script => (
                i18n_text!(keys::editor::menu::SCRIPT, "脚本"),
                IconName::FileText,
            ),
            EditorRoute::Stage => (
                i18n_text!(keys::editor::menu::STAGE, "舞台"),
                IconName::Monitor,
            ),
            EditorRoute::Character => (
                i18n_text!(keys::editor::menu::CHARACTER, "角色"),
                IconName::UserRound,
            ),
        }
    }
}
