mod view;

use gpui::{App, Application, WindowOptions};
use view::editor::Editor;

fn main() {
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);

        cx.open_window(WindowOptions::default(), |window, cx| {
            Editor::view(window, cx)
        })
        .unwrap();
    });
}
