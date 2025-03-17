use editor::Editor;
use gpui::{App, Application, WindowOptions};

fn main() {
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);
        db::init().unwrap();

        cx.open_window(WindowOptions::default(), |window, cx| {
            Editor::view(window, cx)
        })
        .unwrap();
    });
}
