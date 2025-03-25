use std::rc::Rc;

use kernel::{model::note::UpdateNoteBodyEvent, repository::note_repository::NoteRepository};

use crate::model::note::Note;

#[derive(Clone)]
pub struct NoteHandler {
    repository: Rc<dyn NoteRepository>,
}

impl NoteHandler {
    pub fn new(repository: Rc<dyn NoteRepository>) -> Self {
        Self { repository }
    }

    pub fn get_all(&self) -> Vec<Note> {
        match self.repository.get_notes() {
            Ok(notes) => notes.into_iter().map(Note::from).collect(),
            Err(_) => vec![],
        }
    }

    pub fn update_note_body(&self, event: UpdateNoteBodyEvent) {
        self.repository.update_note_body(event).unwrap();
    }
}
