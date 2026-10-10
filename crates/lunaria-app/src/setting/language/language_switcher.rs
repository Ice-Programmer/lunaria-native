use crate::app_services::AppServices;
use crate::window_root::get_language_switcher;
use gpui_kit::base::v_flex;
use gpui_kit::component::notification::Notification;
use gpui_kit::component::{ActiveTheme, WindowExt};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    App, Context, Div, FontWeight, IntoElement, ParentElement, Render, Styled, Task, Window, div,
    px,
};
use lunaria_core::settings::language::model::Language;
use lunaria_ui::extensions::focus::FocusExt;
use lunaria_ui::i18n::locale;
use lunaria_ui::i18n::locale::LocaleState;
use std::time::Duration;

pub struct LanguageSwitcher {
    selected_language: Option<Language>,
    commit_timer: Option<Task<()>>,
}

impl LanguageSwitcher {
    pub fn new() -> Self {
        Self {
            selected_language: None,
            commit_timer: None,
        }
    }

    pub fn show(window: &mut Window, cx: &mut App) {
        let Some(switcher) = get_language_switcher(window, cx) else {
            return;
        };

        switcher.update(cx, |this, cx| {
            this.next(cx);
        });
    }

    fn next(&mut self, cx: &mut Context<Self>) {
        let current_locale = self
            .selected_language
            .map(Language::locale)
            .unwrap_or_else(|| cx.global::<LocaleState>().locale.as_str());

        let next_language = Language::ALL
            .iter()
            .position(|language| language.locale() == current_locale)
            .map(|index| Language::ALL[(index + 1) % Language::ALL.len()])
            .unwrap_or_default();

        self.selected_language = Some(next_language);
        self.commit_timer = None;

        self.commit_timer = Some(cx.spawn(async |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(600))
                .await;

            let _ = this.update(cx, |this, cx| {
                this.commit(cx);
            });
        }));

        cx.notify();
    }

    fn commit(&mut self, cx: &mut Context<Self>) {
        let Some(language) = self.selected_language else {
            return;
        };

        self.close(cx);

        AppServices::run(
            cx,
            |services| async move {
                services
                    .language_service
                    .set(language)
                    .await
                    .map(|()| language)
            },
            |_, result, cx| match result {
                Ok(language) => {
                    locale::set_locale(language.locale(), cx);
                }
                Err(error) => {
                    if let Some(handle) = cx.active_window() {
                        let _ = handle.update(cx, |_, window, cx| {
                            window.push_notification(
                                Notification::error(format!("切换界面语言失败 {}", error)),
                                cx,
                            );
                        });
                    }
                }
            },
        );
    }

    fn close(&mut self, cx: &mut Context<Self>) {
        self.selected_language = None;
        self.commit_timer = None;
        cx.notify()
    }

    pub fn render_panel(&self, cx: &mut Context<Self>) -> Div {
        let Some(selected_language) = self.selected_language else {
            return div();
        };

        v_flex()
            .w(px(240.))
            .max_w_full()
            .gap_1()
            .p_3()
            .rounded_lg()
            .border_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().popover)
            .text_color(cx.theme().popover_foreground)
            .blur_on_mouse_down_out()
            .child(
                div()
                    .px_3()
                    .pb_2()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("界面语言"),
            )
            .children(Language::ALL.iter().map(|language| {
                let selected = language.locale() == selected_language.locale();

                div()
                    .px_3()
                    .py_2()
                    .rounded_md()
                    .text_sm()
                    .when(selected, |row| {
                        row.bg(cx.theme().sidebar_accent)
                            .text_color(cx.theme().sidebar_accent_foreground)
                            .font_weight(FontWeight::SEMIBOLD)
                    })
                    .child(language.label())
            }))
    }
}

impl Render for LanguageSwitcher {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .p_4()
            .child(self.render_panel(cx))
            .into_any_element()
    }
}
