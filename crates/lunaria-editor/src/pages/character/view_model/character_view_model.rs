use gpui_kit::component::WindowExt;
use gpui_kit::{Context, Entity, Window};
use lunaria_app::app_services::AppServices;
use lunaria_core::character::model::Character;
use lunaria_core::character::service::CharacterService;
use lunaria_ui::components::custom_notification::Notification;

pub struct CharacterViewModel {
    character_list: Vec<Character>,
    loading: bool,
    character_service: CharacterService,
}

impl CharacterViewModel {
    pub fn subscribe<T: 'static>(view_model: &Entity<Self>, cx: &mut Context<T>) {
        cx.observe(view_model, |_, _, cx| cx.notify()).detach();
    }

    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            character_list: Vec::new(),
            loading: false,
            character_service: AppServices::require_service(cx),
        }
    }

    fn set_loading(&mut self, cx: &mut Context<Self>, loading: bool) {
        self.loading = loading;
        cx.notify();
    }

    pub fn get_total(&self) -> usize {
        self.character_list.len()
    }

    pub fn list_character(&mut self, cx: &mut Context<Self>) {
        if self.loading {
            return;
        }

        self.set_loading(cx, true);

        let service = self.character_service.clone();

        AppServices::run_future(
            cx,
            async move { service.list().await },
            |vm, result, window, cx| {
                match result {
                    Ok(characters) => vm.character_list = characters,
                    Err(message) => {
                        window.push_notification(Notification::error(message), cx);
                    }
                }

                vm.set_loading(cx, false);
            },
        )
    }
}
