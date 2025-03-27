use gpui::{App, Application};

fn main() {
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);
        theme::init(cx);
        menu::init(cx);
        registry::init(cx);
        window::init(cx);

        cx.activate(true);
    });
}
