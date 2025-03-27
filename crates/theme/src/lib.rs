use gpui::App;
use gpui_component::{Theme, ThemeColor, black};

pub fn init(cx: &mut App) {
    let theme = Theme::from(ThemeColor {
        foreground: black(),
        ..ThemeColor::light()
    });

    cx.set_global(theme);
}
