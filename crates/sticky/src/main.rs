use gpui::{App, Application, black};
use gpui_component::{Theme, ThemeColor};

fn main() {
    Application::new().run(|cx: &mut App| {
        //cx.activate(true);

        gpui_component::init(cx);

        let theme = Theme::from(ThemeColor {
            foreground: black(),
            ..ThemeColor::light()
        });

        cx.set_global(theme);

        registry::init(cx);
        window::init(cx);
    });
}
