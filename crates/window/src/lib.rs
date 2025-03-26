use editor_delegate::EditorDelegate;
use gpui::App;

pub mod editor;
pub mod editor_delegate;
pub mod list;
mod window_options;

pub fn init(cx: &mut App) {
    editor::init(cx);

    EditorDelegate::new().render_notes(cx);
}
