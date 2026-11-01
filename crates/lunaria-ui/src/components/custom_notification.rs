use gpui_kit::base::h_flex;
use gpui_kit::component::notification::{Notification as BaseNotification, NotificationType};
use gpui_kit::component::{ActiveTheme, Icon, IconName};
use gpui_kit::{IntoElement, ParentElement, SharedString, Styled, div};

pub struct Notification;

// 中文场景下原生 Notification icon 和 title 没有居中对齐，所以单独写了一个适配该场景
impl Notification {
    pub fn success(message: impl Into<SharedString>) -> BaseNotification {
        Self::build(message, NotificationType::Success)
    }

    pub fn error(message: impl Into<SharedString>) -> BaseNotification {
        Self::build(message, NotificationType::Error)
    }

    pub fn warning(message: impl Into<SharedString>) -> BaseNotification {
        Self::build(message, NotificationType::Warning)
    }

    pub fn info(message: impl Into<SharedString>) -> BaseNotification {
        Self::build(message, NotificationType::Info)
    }

    fn build(message: impl Into<SharedString>, kind: NotificationType) -> BaseNotification {
        let message = message.into();

        BaseNotification::new().content(move |_, _, cx| {
            let (icon, color) = match kind {
                NotificationType::Success => (IconName::CircleCheck, cx.theme().success),
                NotificationType::Error => (IconName::CircleX, cx.theme().danger),
                NotificationType::Warning => (IconName::TriangleAlert, cx.theme().warning),
                NotificationType::Info => (IconName::Info, cx.theme().info),
            };

            h_flex()
                .w_full()
                .min_w_0()
                .items_center()
                .gap_2()
                .child(Icon::new(icon).size_4().flex_shrink_0().text_color(color))
                .child(div().flex_1().min_w_0().text_sm().child(message.clone()))
                .into_any_element()
        })
    }
}
