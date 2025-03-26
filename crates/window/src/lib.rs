use editor_delegater::EditorDelegater;
use gpui::App;

pub mod editor;
pub mod editor_delegater;
pub mod list;
mod window_options;

pub fn init(cx: &mut App) {
    editor::init(cx);

    EditorDelegater::new(cx).render_notes(cx);
}
