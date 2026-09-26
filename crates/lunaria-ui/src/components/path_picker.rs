use crate::components::custom_button::CustomButton;
use gpui::base::input::{InputEvent, InputState};
use gpui::base::{Disableable, h_flex};
use gpui::{App, ElementId, Entity, IntoElement, RenderOnce, Styled, WeakEntity, Window};
use gpui_kit as gpui;
use gpui_kit::assets::IconName;
use gpui_kit::component::WindowExt;
use gpui_kit::component::input::Input;
use gpui_kit::component::notification::Notification;
use gpui_kit::{Div, InteractiveElement, ParentElement, StyleRefinement, div};
use rfd::{AsyncFileDialog, FileHandle};
use std::path::PathBuf;
use std::pin::Pin;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PathPickerMode {
    File,
    #[default]
    Directory,
}

#[derive(IntoElement)]
pub struct PathPicker {
    id: ElementId,
    input: Entity<InputState>,
    mode: PathPickerMode,
    base: Div,
    disabled: bool,
}

impl PathPicker {
    pub fn new(id: impl Into<ElementId>, input: &Entity<InputState>) -> Self {
        Self {
            base: h_flex().w_full().min_w_0().gap_2(),
            id: id.into(),
            input: input.clone(),
            mode: PathPickerMode::default(),
            disabled: false,
        }
    }

    pub fn mode(mut self, mode: PathPickerMode) -> Self {
        self.mode = mode;
        self
    }

    fn browse(
        input: WeakEntity<InputState>,
        picking: WeakEntity<bool>,
        mode: PathPickerMode,
        window: &mut Window,
        cx: &mut App,
    ) {
        let (Some(input_state), Some(picking_state)) = (input.upgrade(), picking.upgrade()) else {
            return;
        };

        // 防止重复触发
        if *picking_state.read(cx) {
            return;
        }

        let mut directory = PathBuf::from(input_state.read(cx).value().as_ref());

        if mode == PathPickerMode::File && directory.is_file() {
            directory.pop();
        };

        let dialog = AsyncFileDialog::new()
            .set_parent(&*window)
            .set_directory(directory);

        // 更新选择状态
        picking_state.update(cx, |pending, cx| {
            *pending = true;
            cx.notify();
        });

        // 创建路径选择窗器
        let selection: Pin<Box<dyn Future<Output = Option<FileHandle>>>> = match mode {
            PathPickerMode::File => Box::pin(dialog.pick_file()),
            PathPickerMode::Directory => Box::pin(dialog.pick_folder()),
        };

        // 打开文件选择窗口
        window
            .spawn(cx, async move |cx| {
                let selected = selection.await;

                let _ = cx.update(|window, cx| {
                    // 需要重新判断 picking 状态
                    let Some(picking_state) = picking.upgrade() else {
                        return;
                    };

                    picking_state.update(cx, |pending, cx| {
                        *pending = false;
                        cx.notify();
                    });

                    let (Some(input), Some(selected)) = (input.upgrade(), selected) else {
                        return;
                    };

                    let Some(value) = selected.path().to_str() else {
                        window.push_notification(
                            Notification::error(
                                "The selected path cannot be displayed as a single line of text",
                            ),
                            cx,
                        );
                        return;
                    };

                    // 更新路径
                    input.update(cx, |state, cx| {
                        let previous = state.value();
                        if previous.as_ref() == value {
                            return;
                        }
                        state.set_value(value.to_owned(), window, cx);

                        if state.value() != previous {
                            cx.emit(InputEvent::Change);
                        }
                    });
                });
            })
            .detach();
    }
}

impl Disableable for PathPicker {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Styled for PathPicker {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for PathPicker {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let picking = window.use_keyed_state(self.id.clone(), cx, |_, _| false);
        let is_picking = *picking.read(cx);
        let disabled = self.disabled || is_picking;

        let input = self.input.downgrade();
        let picking = picking.downgrade();
        let mode = self.mode;

        self.base
            .id(self.id)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .child(Input::new(&self.input).w_full().disabled(disabled)),
            )
            .child(
                CustomButton::new("browse")
                    .label("浏览")
                    .icon(IconName::Folder)
                    .flex_shrink_0()
                    .disabled(disabled)
                    .on_click(move |_, window, cx| {
                        Self::browse(input.clone(), picking.clone(), mode, window, cx);
                    }),
            )
    }
}
