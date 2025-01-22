use gpui::{
    div, rgb, App, ParentElement, Render, SharedString, Styled, VisualContext, WindowOptions,
};

struct Hello {
    text: SharedString,
}

impl Render for Hello {
    fn render(&mut self, _cx: &mut gpui::ViewContext<Self>) -> impl gpui::IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
    }
}

fn main() {
    let app = App::new();
    app.run(|cx| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| Hello {
                text: "World".into(),
            })
        })
        .unwrap();
    });
}
