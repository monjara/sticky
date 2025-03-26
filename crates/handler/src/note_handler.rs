use std::rc::Rc;

use kernel::{
    model::note::{UpdateNoteActiveEvent, UpdateNoteBodyEvent, UpdateNoteBoundsEvent},
    repository::note_repository::NoteRepository,
};

use crate::model::note::Note;

#[derive(Clone)]
pub struct NoteHandler {
    repository: Rc<dyn NoteRepository>,
}

impl NoteHandler {
    pub fn new(repository: Rc<dyn NoteRepository>) -> Self {
        Self { repository }
    }

    pub fn create_note(&self) -> Note {
        let note = self.repository.create_note().unwrap();
        Note::from(note)
    }

    pub fn get_all(&self) -> Vec<Note> {
        match self.repository.get_notes() {
            Ok(notes) => notes.into_iter().map(Note::from).collect(),
            Err(_) => vec![],
        }
    }

    pub fn get_by_id(&self, id: &str) -> Option<Note> {
        if let Ok(note) = self.repository.get_note_by_id(id) {
            note.map(Note::from)
        } else {
            None
        }
    }

    pub fn update_note_body(&self, event: UpdateNoteBodyEvent) {
        self.repository.update_note_body(event).unwrap();
    }

    pub fn update_note_bounds(&self, event: UpdateNoteBoundsEvent) {
        self.repository.update_note_bounds(event).unwrap();
    }

    pub fn toggle_note_active(&self, id: &str) {
        if let Some(note) = self.repository.get_note_by_id(id).unwrap() {
            self.repository
                .update_note_active(UpdateNoteActiveEvent {
                    id: note.id,
                    is_active: !note.is_active,
                })
                .unwrap();
        }
    }

    pub fn delete_note(&self, id: &str) {
        self.repository.delete_note_by_id(id).unwrap();
    }
}
