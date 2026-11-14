use crate::route::editor_route::EditorRoute;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{ActiveTheme, Icon, Selectable, v_flex};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{Context, Div, IntoElement, ParentElement, Render, Styled, Window, div, px};

const SIDEBAR_ITEM_WIDTH: f32 = 40.0;
const SIDEBAR_WIDTH: f32 = 50.0;

pub struct EditorSidebar {
    route: EditorRoute,
}

impl EditorSidebar {
    pub fn new(route: EditorRoute) -> Self {
        Self { route }
    }

    pub fn route(&self) -> EditorRoute {
        self.route
    }

    fn is_current_route(&self, route: EditorRoute) -> bool {
        self.route == route
    }

    fn set_current_route(&mut self, route: EditorRoute, cx: &mut Context<Self>) {
        if self.is_current_route(route) {
            return;
        }

        self.route = route;
        cx.notify();
    }

    fn render_divider(&self, cx: &mut Context<Self>) -> Div {
        div()
            .mx(px(8.))
            .h(px(1.))
            .my_2()
            .bg(cx.theme().sidebar_border)
    }

    fn render_item(&self, route: EditorRoute, cx: &mut Context<Self>) -> impl IntoElement {
        let (label, icon) = route.label_and_icon();

        div()
            .relative()
            .size(px(SIDEBAR_ITEM_WIDTH))
            .child(
                Button::new(label.clone())
                    .ghost()
                    .size_full()
                    .selected(self.is_current_route(route))
                    .when(self.is_current_route(route), |button| {
                        button.bg(cx.theme().sidebar_accent)
                    })
                    .child(Icon::new(icon))
                    .tooltip(label)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.set_current_route(route, cx);
                    })),
            )
            .when(self.is_current_route(route), |item| {
                item.child(
                    div()
                        .absolute()
                        .left_0()
                        .top(px(6.))
                        .bottom(px(6.))
                        .w(px(2.))
                        .rounded_full()
                        .bg(cx.theme().primary),
                )
            })
    }
}

impl Render for EditorSidebar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w(px(SIDEBAR_WIDTH))
            .h_full()
            .bg(cx.theme().sidebar)
            .py_2()
            .child(
                v_flex()
                    .items_center()
                    .gap(px(4.))
                    .child(self.render_item(EditorRoute::Script, cx))
                    .child(self.render_item(EditorRoute::Stage, cx)),
            )
            .child(self.render_divider(cx))
            .child(
                v_flex()
                    .items_center()
                    .gap(px(4.))
                    .child(self.render_item(EditorRoute::Character, cx)),
            )
    }
}
