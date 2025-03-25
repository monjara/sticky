use std::error::Error;

use crate::model::note::{CreateNoteEvent, Note, UpdateNoteBodyEvent, UpdateNoteTitleEvent};

pub trait NoteRepository {
    fn get_notes(&self) -> Result<Vec<Note>, Box<dyn Error>>;
    fn create_note(&self, note: CreateNoteEvent) -> Result<Note, Box<dyn Error>>;
    fn update_note_title(&self, event: UpdateNoteTitleEvent) -> Result<String, Box<dyn Error>>;
    fn update_note_body(&self, event: UpdateNoteBodyEvent) -> Result<String, Box<dyn Error>>;
    fn delete_note_by_id(&self, id: i32) -> Result<(), Box<dyn Error>>;
}
