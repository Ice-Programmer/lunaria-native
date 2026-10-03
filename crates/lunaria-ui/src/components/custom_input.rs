use gpui_kit::base::input::Escape;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::{Disableable, FocusableExt, RoleOverride, Sizable, Size};
use gpui_kit::{
    App, Div, ElementId, Entity, EntityInputHandler, Focusable, InteractiveElement, Interactivity,
    IntoElement, Modifiers, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement,
    StyleRefinement, Styled, Window, div,
};

#[derive(IntoElement)]
pub struct CustomInput {
    base: Div,
    wrapper_id: ElementId,
    inner: Input,
    state: Entity<InputState>,
    blur_on_mouse_down_out: bool,
    blur_on_escape: bool,
}

impl CustomInput {
    pub fn new(state: &Entity<InputState>) -> Self {
        Self {
            base: div().min_w_0(),
            wrapper_id: ("custom-input", state.entity_id()).into(),
            inner: Input::new(state).w_full(),
            state: state.clone(),
            blur_on_mouse_down_out: true,
            blur_on_escape: true,
        }
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        let id = id.into();
        self.wrapper_id = (id.clone(), "wrapper").into();
        self.inner = self.inner.id(id);
        self
    }

    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.inner = self.inner.aria_label(label);
        self
    }

    pub fn accessibility_id(mut self, id: impl Into<SharedString>) -> Self {
        self.inner = self.inner.accessibility_id(id);
        self
    }

    pub fn role(mut self, role: impl Into<RoleOverride>) -> Self {
        self.inner = self.inner.role(role);
        self
    }

    pub fn prefix(mut self, prefix: impl IntoElement) -> Self {
        self.inner = self.inner.prefix(prefix);
        self
    }

    pub fn suffix(mut self, suffix: impl IntoElement) -> Self {
        self.inner = self.inner.suffix(suffix);
        self
    }

    pub fn cleanable(mut self, cleanable: bool) -> Self {
        self.inner = self.inner.cleanable(cleanable);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.inner = self.inner.disabled(disabled);
        self
    }

    pub fn readonly(mut self, readonly: bool) -> Self {
        self.inner = self.inner.readonly(readonly);
        self
    }

    pub fn appearance(mut self, appearance: bool) -> Self {
        self.inner = self.inner.appearance(appearance);
        self
    }

    pub fn bordered(mut self, bordered: bool) -> Self {
        self.inner = self.inner.bordered(bordered);
        self
    }

    pub fn focus_bordered(mut self, bordered: bool) -> Self {
        self.inner = self.inner.focus_bordered(bordered);
        self
    }

    pub fn tab_index(mut self, index: isize) -> Self {
        self.inner = self.inner.tab_index(index);
        self
    }

    pub fn blur_on_mouse_down_out(mut self, enabled: bool) -> Self {
        self.blur_on_mouse_down_out = enabled;
        self
    }

    pub fn blur_on_escape(mut self, enabled: bool) -> Self {
        self.blur_on_escape = enabled;
        self
    }

    pub fn with_input(mut self, configure: impl FnOnce(Input) -> Input) -> Self {
        self.inner = configure(self.inner);
        self
    }
}

impl Styled for CustomInput {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for CustomInput {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for CustomInput {}

impl Disableable for CustomInput {
    fn disabled(self, disabled: bool) -> Self {
        Self::disabled(self, disabled)
    }
}

impl Sizable for CustomInput {
    fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.inner = self.inner.with_size(size);
        self
    }
}

impl FocusableExt for CustomInput {
    fn focus_ring(mut self, enabled: bool) -> Self {
        self.inner = self.inner.focus_ring(enabled);
        self
    }

    fn is_focus_ring_enabled(&self) -> bool {
        self.inner.is_focus_ring_enabled()
    }
}

impl RenderOnce for CustomInput {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus = self.state.focus_handle(cx);
        let mut base = self.base.id(self.wrapper_id);

        if self.blur_on_mouse_down_out {
            let focus = focus.clone();
            base = base.on_mouse_down_out(move |_, window, cx| {
                if focus.is_focused(window) {
                    window.blur(cx);
                }
            });
        }

        if self.blur_on_escape {
            let state = self.state;
            let composition_state = state.clone();
            let composition_focus = focus.clone();
            base = base.capture_action(move |_: &Escape, window, cx| {
                if !composition_focus.is_focused(window) {
                    return;
                }
                let composing = composition_state.update(cx, |input, cx| {
                    if input.marked_text_range(window, cx).is_none() {
                        return false;
                    }
                    input.unmark_text(window, cx);
                    cx.notify();
                    true
                });
                if composing {
                    cx.stop_propagation();
                }
            });
            base = base.capture_key_down(move |event, window, cx| {
                if event.keystroke.key != "escape"
                    || event.keystroke.modifiers != Modifiers::default()
                    || !focus.is_focused(window)
                {
                    return;
                }

                // 是否在拼写
                let composing = state.update(cx, |input, cx| {
                    input.marked_text_range(window, cx).is_some()
                });
                if composing {
                    return;
                }

                window.blur(cx);
                cx.stop_propagation();
            });
        }

        base.child(self.inner)
    }
}
