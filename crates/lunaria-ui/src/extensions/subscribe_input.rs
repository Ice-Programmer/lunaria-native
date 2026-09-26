use gpui_kit::base::input::{InputEvent, InputState};
use gpui_kit::{Context, Entity, Subscription};

pub fn subscribe_input<T: 'static>(
    input: &Entity<InputState>,
    cx: &mut Context<T>,
) -> Subscription {
    cx.subscribe(input, |_, _, event, cx| {
        if matches!(event, InputEvent::Change) {
            cx.notify();
        }
    })
}
