use gpui_kit::{Div, Styled, div, px};

pub trait SpaceExt {
    fn horizontal_space(self) -> Div;
    fn vertical_space(self) -> Div;
}

impl SpaceExt for u32 {
    fn horizontal_space(self) -> Div {
        div().w(px(self as f32)).flex_shrink_0()
    }

    fn vertical_space(self) -> Div {
        div().h(px(self as f32)).flex_shrink_0()
    }
}