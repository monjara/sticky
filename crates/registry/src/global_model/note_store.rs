use gpui::Global;
pub use handler::model::note::Note;

#[derive(Clone)]
pub struct NoteStore {
    pub notes: Vec<Note>,
    pub new_notes: Vec<Note>,
}

impl Global for NoteStore {}
