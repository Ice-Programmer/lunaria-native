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
