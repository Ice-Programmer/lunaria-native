use gpui_kit::component::kbd::Kbd;
use gpui_kit::{Context, Entity, EventEmitter, Subscription, Window};
use lunaria_app::app_services::AppServices;
use lunaria_app::shortcuts::to_keystroke;
use lunaria_core::settings::shortcuts::model::{ShortcutCategory, ShortcutItem};

pub enum ShortcutEvent {
    Started,
    Loaded,
    FilterChanged,
    Failed(String),
}

#[derive(Default)]
pub struct ShortcutViewModel {
    items: Vec<ShortcutItem>,
    query: String,
    modified_only: bool,
    loading: bool,
}

impl EventEmitter<ShortcutEvent> for ShortcutViewModel {}

impl ShortcutViewModel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe<T: 'static>(
        view_model: &Entity<Self>,
        window: &mut Window,
        cx: &mut Context<T>,
    ) -> Subscription {
        cx.subscribe_in(view_model, window, |_, _, _, _, cx| cx.notify())
    }

    pub fn is_loading(&self) -> bool {
        self.loading
    }

    pub fn items(&self) -> &[ShortcutItem] {
        &self.items
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn total_count(&self) -> usize {
        self.items.len()
    }

    pub fn modified_count(&self) -> usize {
        self.items.iter().filter(|item| item.is_modified()).count()
    }

    pub fn modified_only(&self) -> bool {
        self.modified_only
    }

    pub fn set_query(&mut self, query: &str, cx: &mut Context<Self>) {
        let query = query.trim().to_lowercase();

        if self.query == query {
            return;
        }

        self.query = query;
        cx.emit(ShortcutEvent::FilterChanged);
    }

    pub fn set_modified_only(&mut self, modified_only: bool, cx: &mut Context<Self>) {
        if self.modified_only == modified_only {
            return;
        }

        self.modified_only = modified_only;
        cx.emit(ShortcutEvent::FilterChanged);
    }

    pub fn load_shortcut_list(&mut self, cx: &mut Context<Self>) {
        if self.loading {
            return;
        }

        self.loading = true;
        cx.emit(ShortcutEvent::Started);

        AppServices::run(
            cx,
            |services| async move { services.shortcut_service.list().await },
            |this, result, cx| {
                this.loading = false;

                let event = match result {
                    Ok(items) => {
                        this.items = items;
                        ShortcutEvent::Loaded
                    }
                    Err(error) => ShortcutEvent::Failed(error),
                };

                cx.emit(event);
            },
        )
    }

    pub fn categories(&self) -> Vec<ShortcutCategory> {
        let mut categories = Vec::new();

        for item in &self.items {
            let category = item.definition.category;

            if !categories.contains(&category) {
                categories.push(category);
            }
        }

        categories
    }

    pub fn filtered_items(
        &self,
        category: ShortcutCategory,
    ) -> impl Iterator<Item = &ShortcutItem> + '_ {
        self.items
            .iter()
            .filter(move |item| item.definition.category == category)
            .filter(|item| !self.modified_only || item.is_modified())
            .filter(|item| self.matches_query(item))
    }

    fn matches_query(&self, item: &ShortcutItem) -> bool {
        if self.query.is_empty() {
            return true;
        }

        let mut search_text = format!(
            "{} {} {}",
            item.definition.title,
            item.definition.category.label(),
            item.definition.category.description()
        );

        for shortcut in item.custom.as_deref().unwrap_or(&[]) {
            let stroke = to_keystroke(shortcut);

            search_text.push(' ');
            search_text.push_str(&Kbd::format(&stroke));
            search_text.push(' ');
            search_text.push_str(&stroke.unparse());
        }

        search_text.to_lowercase().contains(&self.query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lunaria_core::settings::shortcuts::model::{
        Shortcut, ShortcutAction, ShortcutDefinition, ShortcutModifiers,
    };

    fn fixture() -> ShortcutViewModel {
        let unchanged = |action, title, category, key| {
            let shortcut = Shortcut::new(key, ShortcutModifiers::default());
            ShortcutItem {
                definition: ShortcutDefinition {
                    action,
                    title,
                    category,
                    defaults: vec![shortcut.clone()],
                },
                custom: Some(vec![shortcut]),
            }
        };

        ShortcutViewModel {
            items: vec![
                unchanged(
                    ShortcutAction::OpenSettings,
                    "打开设置窗口",
                    ShortcutCategory::Application,
                    ",",
                ),
                ShortcutItem {
                    definition: ShortcutDefinition {
                        action: ShortcutAction::ToggleTheme,
                        title: "切换主题",
                        category: ShortcutCategory::Application,
                        defaults: vec![Shortcut::new("f1", ShortcutModifiers::default())],
                    },
                    custom: Some(vec![Shortcut::new(
                        "f2",
                        ShortcutModifiers {
                            shift: true,
                            ..ShortcutModifiers::default()
                        },
                    )]),
                },
                ShortcutItem {
                    definition: ShortcutDefinition {
                        action: ShortcutAction::CloseWindow,
                        title: "关闭当前窗口",
                        category: ShortcutCategory::Editor,
                        defaults: vec![Shortcut::new("w", ShortcutModifiers::default())],
                    },
                    custom: None,
                },
                unchanged(
                    ShortcutAction::Quit,
                    "退出应用",
                    ShortcutCategory::Launcher,
                    "q",
                ),
            ],
            ..ShortcutViewModel::default()
        }
    }

    fn visible_actions(
        view_model: &ShortcutViewModel,
        category: ShortcutCategory,
    ) -> Vec<ShortcutAction> {
        view_model
            .filtered_items(category)
            .map(|item| item.definition.action)
            .collect()
    }

    #[test]
    fn summary_counts_include_changed_and_disabled_actions() {
        let mut view_model = fixture();
        assert_eq!(view_model.total_count(), 4);
        assert_eq!(view_model.modified_count(), 2);
        assert_eq!(
            visible_actions(&view_model, ShortcutCategory::Application),
            [ShortcutAction::OpenSettings, ShortcutAction::ToggleTheme]
        );

        view_model.modified_only = true;
        assert_eq!(
            visible_actions(&view_model, ShortcutCategory::Application),
            [ShortcutAction::ToggleTheme]
        );
        assert_eq!(
            visible_actions(&view_model, ShortcutCategory::Editor),
            [ShortcutAction::CloseWindow]
        );
        assert!(visible_actions(&view_model, ShortcutCategory::Launcher).is_empty());
        assert_eq!(view_model.total_count(), 4);
        assert_eq!(view_model.modified_count(), 2);
    }

    #[test]
    fn search_combines_title_category_effective_key_and_modified_filter() {
        let mut view_model = fixture();
        view_model.query = "主题".into();
        assert_eq!(
            visible_actions(&view_model, ShortcutCategory::Application),
            [ShortcutAction::ToggleTheme]
        );

        view_model.query = "shift-f2".into();
        view_model.modified_only = true;
        assert_eq!(
            visible_actions(&view_model, ShortcutCategory::Application),
            [ShortcutAction::ToggleTheme]
        );
        assert!(visible_actions(&view_model, ShortcutCategory::Editor).is_empty());

        view_model.query = "f1".into();
        assert!(visible_actions(&view_model, ShortcutCategory::Application).is_empty());

        view_model.query = "关闭".into();
        assert_eq!(
            visible_actions(&view_model, ShortcutCategory::Editor),
            [ShortcutAction::CloseWindow]
        );
        view_model.query = "w".into();
        assert!(visible_actions(&view_model, ShortcutCategory::Editor).is_empty());

        view_model.query.clear();
        assert_eq!(
            visible_actions(&view_model, ShortcutCategory::Application),
            [ShortcutAction::ToggleTheme]
        );
        assert_eq!(
            visible_actions(&view_model, ShortcutCategory::Editor),
            [ShortcutAction::CloseWindow]
        );

        view_model.modified_only = false;
        assert_eq!(
            visible_actions(&view_model, ShortcutCategory::Application),
            [ShortcutAction::OpenSettings, ShortcutAction::ToggleTheme]
        );
        view_model.query = "项目创建".into();
        assert_eq!(
            visible_actions(&view_model, ShortcutCategory::Launcher),
            [ShortcutAction::Quit]
        );
    }
}
