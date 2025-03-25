mod window_options;

use editor::Editor;
use gpui::App;
use registry::global_model::note_store::NoteStore;
use window_options::{location::Location, make_editor_option, window_size::WindowSize};

pub mod editor;
pub mod list;

pub fn init(cx: &mut App) {
    let notes = cx.global::<NoteStore>().notes.clone();

    if notes.is_empty() {
        //cx.open_window(window_options(), |window, cx| Editor::view(window, cx, &id))
        //    .unwrap();
    }

    for note in notes {
        let location = Location::new(note.location_x, note.location_y);
        let size = WindowSize::new(note.width, note.height);
        cx.open_window(make_editor_option(location, size), |window, cx| {
            Editor::view(window, cx, &note.id)
        })
        .unwrap();
    }
}
