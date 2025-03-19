use std::collections::HashMap;

use gpui::Global;
use handler::model::note::Note;

#[derive(Clone)]
pub struct NoteStore {
    pub notes: Vec<Note>,
    pub note_accessor: HashMap<String, Note>,
}

impl Global for NoteStore {}
