use crate::shortcuts::i18n::category_i18n::ShortcutCategoryI18nExt;
use crate::shortcuts::i18n::definition_i18n::ShortcutDefinitionI18nExt;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::kbd::Kbd;
use gpui_kit::component::{ActiveTheme, WindowExt};
use gpui_kit::{Anchor, Context, Entity, EventEmitter, Styled, Subscription, Window};
use lunaria_app::app_services::AppServices;
use lunaria_app::setting::shortcuts::{bindings, to_keystroke};
use lunaria_core::settings::error::SettingError;
use lunaria_core::settings::shortcuts::model::{
    Shortcut, ShortcutAction, ShortcutCategory, ShortcutItem,
};
use lunaria_ui::components::custom_notification::Notification;

pub enum ShortcutEvent {
    Changed,
    Failed(String),
    Saved,
    ResetAll,
    UndoFinished { restored: bool },
}

struct ResetAllNotification;

#[derive(Default)]
pub struct ShortcutViewModel {
    items: Vec<ShortcutItem>,
    query: String,
    modified_only: bool,
    loading: bool,
    saving: bool,
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
        cx.subscribe_in(view_model, window, |_, vm, event, window, cx| {
            cx.notify();

            let notification = match event {
                ShortcutEvent::Changed => return,
                ShortcutEvent::Failed(error) => Notification::error(error.clone()),
                ShortcutEvent::Saved => Notification::success("快捷键已保存"),
                ShortcutEvent::ResetAll => {
                    let view_model = vm.clone();

                    Notification::success("全部快捷键已恢复默认")
                        .id::<ResetAllNotification>()
                        .placement(Anchor::BottomRight)
                        .action(move |_, _, cx| {
                            let view_model = view_model.clone();
                            Button::new("undo-reset-shortcuts")
                                .ghost()
                                .text_color(cx.theme().primary)
                                .label("撤销")
                                .on_click(move |_, _, cx| {
                                    cx.stop_propagation();

                                    view_model.update(cx, |view_model, cx| {
                                        view_model.undo_reset_all_shortcuts(cx);
                                    })
                                })
                        })
                }
                ShortcutEvent::UndoFinished { restored } => {
                    window.remove_notification::<ResetAllNotification>(cx);

                    if *restored {
                        Notification::success("已撤销恢复默认")
                    } else {
                        Notification::warning("关键键已再次修改，无法撤销此次恢复")
                    }
                }
            };
            window.push_notification(notification, cx);
        })
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

    pub fn can_reset_all(&self) -> bool {
        !self.loading
            && !self.saving
            && self.items.iter().any(|item| item.is_modified())
    }

    pub fn set_query(&mut self, query: &str, cx: &mut Context<Self>) {
        let query = query.trim().to_lowercase();

        if self.query == query {
            return;
        }

        self.query = query;
        cx.emit(ShortcutEvent::Changed);
    }

    pub fn set_modified_only(&mut self, modified_only: bool, cx: &mut Context<Self>) {
        if self.modified_only == modified_only {
            return;
        }

        self.modified_only = modified_only;
        cx.emit(ShortcutEvent::Changed);
    }

    pub fn load_shortcut_list(&mut self, cx: &mut Context<Self>) {
        if self.loading {
            return;
        }

        self.loading = true;

        AppServices::run(
            cx,
            |services| async move { services.shortcut_service.list().await },
            |this, result, cx| {
                this.loading = false;

                match result {
                    Ok(items) => {
                        this.items = items;
                        cx.emit(ShortcutEvent::Changed);
                    }
                    Err(error) => cx.emit(ShortcutEvent::Failed(error)),
                };
            },
        )
    }

    pub fn undo_reset_all_shortcuts(&mut self, cx: &mut Context<Self>) {
        if self.loading || self.saving {
            return;
        }

        self.saving = true;
        cx.emit(ShortcutEvent::Changed);

        let keep_alive = cx.entity();

        AppServices::run(
            cx,
            |services| async move {
                let restored = services.shortcut_service.undo_reset_all().await?;
                let items = services.shortcut_service.list().await?;

                Ok::<_, SettingError>((restored, items))
            },
            move |this, result, cx| {
                let _keep_alive = keep_alive;
                this.saving = false;

                match result {
                    Ok((restored, items)) => {
                        bindings::apply(&items, cx);
                        this.items = items;
                        cx.emit(ShortcutEvent::UndoFinished { restored })
                    }
                    Err(error) => cx.emit(ShortcutEvent::Failed(error)),
                }
            },
        )
    }

    pub fn save_shortcut(
        &mut self,
        cx: &mut Context<Self>,
        action: ShortcutAction,
        shortcuts: Vec<Shortcut>,
        window: &mut Window,
    ) {
        if self.saving {
            return;
        }

        self.saving = true;
        let window_handle = window.window_handle();

        AppServices::run(
            cx,
            |services| async move {
                services.shortcut_service.set(action, shortcuts).await?;
                services.shortcut_service.list().await
            },
            move |this, result, cx| {
                this.saving = false;

                match result {
                    Ok(items) => {
                        bindings::apply(&items, cx);

                        this.items = items;

                        cx.emit(ShortcutEvent::Saved);

                        let _ = window_handle.update(cx, |_, window, _| {
                            window.remove_window();
                        });
                    }
                    Err(error) => {
                        cx.emit(ShortcutEvent::Failed(error));
                    }
                }
            },
        );
    }

    pub fn reset_all_shortcuts(&mut self, cx: &mut Context<Self>) {
        if self.loading || self.saving {
            return;
        }

        self.saving = true;
        cx.emit(ShortcutEvent::Changed);

        AppServices::run(
            cx,
            |services| async move {
                services.shortcut_service.reset_all().await?;
                services.shortcut_service.list().await
            },
            |this, result, cx| {
                this.saving = false;

                match result {
                    Ok(items) => {
                        bindings::apply(&items, cx);
                        this.items = items;
                        cx.emit(ShortcutEvent::ResetAll);
                    }
                    Err(error) => {
                        cx.emit(ShortcutEvent::Failed(error));
                    }
                }
            },
        );
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
            item.definition.title_i18n(),
            item.definition.category.label_i18n(),
            item.definition.category.description_i18n()
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
