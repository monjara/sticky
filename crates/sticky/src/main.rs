use gpui::{App, Application};

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.activate(true);

        components::init(cx);
        registry::init(cx);
        window::init(cx);
    });
}
