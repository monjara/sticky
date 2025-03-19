use gpui::{Pixels, Size};

pub struct WindowSize {
    pub width: f32,
    pub height: f32,
}

impl WindowSize {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl From<WindowSize> for Size<Pixels> {
    fn from(value: WindowSize) -> Self {
        Size::new(Pixels(value.width), Pixels(value.height))
    }
}
