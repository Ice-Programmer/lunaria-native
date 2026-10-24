use crate::shortcuts::components::shortcut_keycaps::ShortcutKeycaps;
use gpui_kit::base::h_flex;
use gpui_kit::component::ActiveTheme;
use gpui_kit::{
    Context, EventEmitter, FocusHandle, InteractiveElement, IntoElement, KeystrokeEvent, Modifiers,
    MouseDownEvent, ParentElement, Render, Styled, Subscription, Window, div,
};
use lunaria_core::settings::shortcuts::model::{Shortcut, ShortcutModifiers};

pub struct ShortcutRecorder {
    current_shortcut: Option<Shortcut>,
    focus_handle: FocusHandle,
    _subscriptions: Vec<Subscription>,
}

impl ShortcutRecorder {
    pub fn new(
        current_shortcut: Option<Shortcut>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        // 创建焦点
        let focus_handle = cx.focus_handle();
        let weak = cx.weak_entity();

        // 快捷键监听
        let key_subscription = cx.intercept_keystrokes(move |event, window, cx| {
            let _ = weak.update(cx, |this, cx| {
                this.record_keystroke(event, window, cx);
            });
        });

        // 切换窗口结束录入
        let activation_subscription = cx.observe_window_activation(window, |this, window, cx| {
            if !window.is_window_active() && this.focus_handle.is_focused(window) {
                window.blur(cx);
            };
        });

        Self {
            current_shortcut,
            focus_handle,
            _subscriptions: vec![key_subscription, activation_subscription],
        }
    }

    pub fn value(&self) -> Option<&Shortcut> {
        self.current_shortcut.as_ref()
    }

    /// 点击录入框外，结束录入
    fn on_mouse_down_out(
        &mut self,
        _event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.focus_handle.is_focused(window) {
            window.blur(cx);
        }
    }

    fn record_keystroke(
        &mut self,
        event: &KeystrokeEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.focus_handle.is_focused(window) {
            return;
        }

        // 防止 command + w 关闭窗口
        cx.stop_propagation();

        let stroke = &event.keystroke;
        let modifiers = stroke.modifiers;

        if stroke.key == "escape" && modifiers == Modifiers::default() {
            window.blur(cx);
            return;
        }

        if stroke.key.is_empty() {
            return;
        }

        let shortcut = Shortcut {
            key: stroke.key.clone(),
            modifiers: ShortcutModifiers {
                control: modifiers.control,
                alt: modifiers.alt,
                shift: modifiers.shift,
                platform: modifiers.platform,
                function: modifiers.function,
            },
        };

        if self.current_shortcut.as_ref() == Some(&shortcut) {
            return;
        }

        self.current_shortcut = Some(shortcut.clone());
        cx.notify();
    }

    fn render_content(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focused = self.focus_handle.is_focused(window);

        match &self.current_shortcut {
            Some(shortcut) => ShortcutKeycaps::single(shortcut.clone())
                .flex_wrap()
                .into_any_element(),

            None => div()
                .text_sm()
                .text_color(cx.theme().muted_foreground)
                .child(focused.then_some("请按下组合键").unwrap_or("点击录入"))
                .into_any_element(),
        }
    }
}

impl Render for ShortcutRecorder {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focused = self.focus_handle.is_focused(window);

        h_flex()
            .track_focus(&self.focus_handle)
            .w_full()
            .min_w_0()
            .items_center()
            .px_3()
            .py_2()
            .border_1()
            .border_color(
                focused
                    .then_some(cx.theme().primary)
                    .unwrap_or(cx.theme().input),
            )
            .rounded_lg()
            .bg(cx.theme().background)
            .on_mouse_down_out(cx.listener(Self::on_mouse_down_out))
            .child(self.render_content(window, cx))
    }
}
