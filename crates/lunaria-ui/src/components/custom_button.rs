use gpui_kit::component::button::{Button, ButtonVariant, ButtonVariants};
use gpui_kit::component::{Disableable, Icon, Selectable};
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, px,
};

#[derive(IntoElement)]
pub struct CustomButton {
    inner: Button,
}

impl CustomButton {
    pub const DEFAULT_HEIGHT: f32 = 32.0;
    pub const DEFAULT_HORIZONTAL_PADDING: f32 = 16.0;

    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            inner: Button::new(id)
                .h(px(Self::DEFAULT_HEIGHT))
                .px(px(Self::DEFAULT_HORIZONTAL_PADDING)),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.inner = self.inner.label(label);
        self
    }

    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.inner = self.inner.icon(icon.into());
        self
    }

    pub fn outline(mut self) -> Self {
        self.inner = self.inner.outline();
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.inner = self.inner.loading(loading);
        self
    }

    pub fn tooltip(mut self, text: impl Into<SharedString>) -> Self {
        self.inner = self.inner.tooltip(text);
        self
    }

    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.inner = self.inner.accessibility_label(label);
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.inner = self.inner.on_click(handler);
        self
    }
}

impl ButtonVariants for CustomButton {
    fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.inner = self.inner.with_variant(variant);
        self
    }
}

impl Disableable for CustomButton {
    fn disabled(mut self, disabled: bool) -> Self {
        self.inner = self.inner.disabled(disabled);
        self
    }
}

impl Selectable for CustomButton {
    fn selected(mut self, selected: bool) -> Self {
        self.inner = self.inner.selected(selected);
        self
    }

    fn is_selected(&self) -> bool {
        self.inner.is_selected()
    }
}

impl Styled for CustomButton {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}

impl ParentElement for CustomButton {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.inner.extend(elements);
    }
}

impl RenderOnce for CustomButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.inner
    }
}
