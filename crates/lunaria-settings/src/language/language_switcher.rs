use gpui_kit::{Context, IntoElement, Render, Task, Window, div};
use lunaria_core::settings::language::model::Language;

pub struct LanguageSwitcher {
    selected: Option<Language>,
    commit_timer: Option<Task<()>>,
}

impl LanguageSwitcher {
    pub fn new() -> Self {
        Self {
            selected: None,
            commit_timer: None,
        }
    }
}

impl Render for LanguageSwitcher {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}
