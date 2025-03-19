use editor::{Editor, window_options};
use gpui::App;
use registry::global_model::note_store::NoteStore;

pub mod editor;
pub mod list;

pub fn init(cx: &mut App) {
    let notes = cx.global::<NoteStore>().notes.clone();

    if notes.is_empty() {
        //cx.open_window(window_options(), |window, cx| Editor::view(window, cx, &id))
        //    .unwrap();
    }

    for note in notes {
        cx.open_window(window_options(), |window, cx| {
            Editor::view(window, cx, &note.id)
        })
        .unwrap();
    }
}
