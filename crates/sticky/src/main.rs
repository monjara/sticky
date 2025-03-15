mod view;

use gpui::{App, VisualContext, WindowOptions};
use view::hello::Hello;

fn main() {
    let app = App::new();
    app.run(|cx| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| Hello {
                text: "rust".into(),
            })
        })
        .unwrap();

        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| Hello {
                text: "rust".into(),
            })
        })
        .unwrap();
    });
}
