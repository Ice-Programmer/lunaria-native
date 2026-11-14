use crate::pages::character::character_page::CharacterPage;
use crate::pages::script::script_page::ScriptPage;
use crate::pages::stage::stage_page::StagePage;
use gpui_kit::assets::IconName;
use gpui_kit::{AnyElement, AnyView, App, AppContext, IntoElement, Render, SharedString};
use lunaria_ui::i18n::keys;
use lunaria_ui::i18n_text;

#[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
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

    pub fn build(self, cx: &mut App) -> AnyView {
        match self {
            Self::Script => cx.new(|_| ScriptPage::new()).into(),
            Self::Stage => cx.new(|_| StagePage::new()).into(),
            Self::Character => cx.new(|_| CharacterPage::new()).into(),
        }
    }
}
