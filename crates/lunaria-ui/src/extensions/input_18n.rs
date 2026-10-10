use crate::i18n::locale::{LocaleState, i18n_text};
use gpui_kit::component::input::InputState;
use gpui_kit::{Context, Window};

pub trait InputI18nExt: Sized + 'static {
    fn placeholder_i18n(
        self,
        key: &'static str,
        default_text: &'static str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self;
}

impl InputI18nExt for InputState {
    fn placeholder_i18n(
        self,
        key: &'static str,
        default_text: &'static str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscription = cx.observe_global_in::<LocaleState>(window, move |input, window, cx| {
            input.set_placeholder(i18n_text(key, default_text), window, cx);
        });

        cx.on_release(move |_, _| drop(subscription)).detach();

        self.placeholder(i18n_text(key, default_text))
    }
}
