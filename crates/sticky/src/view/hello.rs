use gpui::{ParentElement, Render, SharedString, Styled, div, rgb};

pub(crate) struct Hello {
    pub(crate) text: SharedString,
}

impl Render for Hello {
    fn render(&mut self, _cx: &mut gpui::ViewContext<Self>) -> impl gpui::IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e_7d32))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xff_ffff))
            .child(format!("Hello, {}!", &self.text))
    }
}
