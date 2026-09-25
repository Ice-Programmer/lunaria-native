use gpui_kit::*;

pub trait FocusExt: Sized {
    fn blur_on_mouse_down_out(self) -> Self;
}

impl FocusExt for Div {
    fn blur_on_mouse_down_out(self) -> Self {
        self.on_mouse_down_out(|_, window, cx| {
            window.blur(cx);
        })
    }
}
