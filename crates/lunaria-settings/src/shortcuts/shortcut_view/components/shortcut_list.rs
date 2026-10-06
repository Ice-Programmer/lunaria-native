use crate::shortcuts::model::{Shortcut, ShortcutCategory, ShortcutItem};
use gpui_kit::base::IndexPath;
use gpui_kit::component::kbd::Kbd;
use gpui_kit::component::list::{ListDelegate, ListItem, ListState};
use gpui_kit::{App, Context, Keystroke, Window};

pub fn is_modified(item: &ShortcutItem) -> bool {
    item.custom.as_deref().unwrap_or(&[]) != item.definition.defaults.as_slice()
}

fn keystroke(shortcut: &Shortcut) -> Keystroke {
    Keystroke {
        key: shortcut.key.clone(),
        modifiers: shortcut.modifiers,
        key_char: None,
    }
}

pub struct ShortcutListDelegate {
    items: Vec<ShortcutItem>,
    query: String,
    modified_only: bool,
    loading: bool,
}

impl ShortcutListDelegate {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            query: String::new(),
            modified_only: false,
            loading: true,
        }
    }

    pub fn set_loading(&mut self, loading: bool) {
        self.loading = loading;
    }

    pub fn is_loading(&self) -> bool {
        self.loading
    }

    pub fn total_count(&self) -> usize {
        self.items.len()
    }

    pub fn modified_count(&self) -> usize {
        self.items.iter().filter(|item| is_modified(item)).count()
    }

    pub fn set_items(&mut self, items: Vec<ShortcutItem>) {
        self.items = items;
        self.loading = false;
    }

    pub fn set_filter(&mut self, query: &str, modified_only: bool) {
        self.query = query.trim().to_lowercase();
        self.modified_only = modified_only;
    }

    pub fn matches_query(&self, item: &ShortcutItem) -> bool {
        if self.query.is_empty() {
            return true;
        }

        let mut search_text = format!(
            "{} {} {}",
            item.definition.title,
            item.definition.category.label(),
            item.definition.category.description(),
        );

        for shortcut in item.custom.as_deref().unwrap_or(&[]) {
            let stroke = keystroke(shortcut);

            search_text.push(' ');
            search_text.push_str(&Kbd::format(&stroke));

            search_text.push(' ');
            search_text.push_str(&stroke.unparse());
        }

        search_text.to_lowercase().contains(&search_text)
    }

    fn categories(&self) -> Vec<ShortcutCategory> {
        let mut categories = Vec::new();

        for item in &self.items {
            let category = item.definition.category;

            if !categories.contains(&category) {
                categories.push(category);
            }
        }

        categories
    }

    fn filtered_items(
        &self,
        category: ShortcutCategory,
    ) -> impl Iterator<Item = &ShortcutItem> + '_ {
        self.items
            .iter()
            .filter(move |item| item.definition.category == category)
            .filter(|item| !self.modified_only || item.is_modified())
            .filter(|item| self.matches_query(item))
    }
}

impl ListDelegate for ShortcutListDelegate {
    type Item = ListItem;

    fn sections_count(&self, _cx: &App) -> usize {
        self.categories().len().max(1)
    }

    fn items_count(&self, section: usize, _cx: &App) -> usize {
        self.categories()
            .get(section)
            .copied()
            .map_or(0, |category| self.filtered_items(category).count())
    }

    fn render_item(
        &mut self,
        ix: IndexPath,
        window: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) -> Option<Self::Item> {
        todo!()
    }

    fn set_selected_index(
        &mut self,
        ix: Option<IndexPath>,
        window: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) {
        todo!()
    }
}
