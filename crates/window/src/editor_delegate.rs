use gpui::App;
use registry::global_model::note_store::{Note, NoteStore};

use crate::{
    editor::Editor,
    window_options::{location::Location, make_editor_option, window_size::WindowSize},
};

pub struct EditorDelegate {}

impl EditorDelegate {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render_notes(&mut self, cx: &mut App) {
        let notes = cx.global::<NoteStore>().notes.clone();
        if notes.is_empty() {
            //cx.open_window(window_options(), |window, cx| Editor::view(window, cx, &id))
            //    .unwrap();
        }

        for note in notes {
            Self::render_note(cx, &note);
        }

        cx.observe_global::<NoteStore>(|cx| {
            let notes = cx.global::<NoteStore>().new_notes.clone();
            for note in notes {
                Self::render_note(cx, &note);
            }
        })
        .detach();
    }

    fn render_note(cx: &mut App, note: &Note) {
        if !note.is_active {
            return;
        }

        let location = Location::new(note.location_x, note.location_y);
        let size = WindowSize::new(note.width, note.height);
        cx.open_window(make_editor_option(location, size), |window, cx| {
            Editor::view(window, cx, &note.id)
        })
        .unwrap();
    }
}

impl Default for EditorDelegate {
    fn default() -> Self {
        Self::new()
    }
}
