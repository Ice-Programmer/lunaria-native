use gpui_kit::{Pixels, px};

#[cfg(target_os = "macos")]
pub const TITLE_BAR_LEFT_PADDING: Pixels = px(80.);
#[cfg(not(target_os = "macos"))]
pub const TITLE_BAR_LEFT_PADDING: Pixels = px(12.);