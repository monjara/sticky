use std::error::Error;

use crate::model::note::{CreateNoteEvent, Note, UpdateNoteBodyEvent, UpdateNoteBoundsEvent};

pub trait NoteRepository {
    fn get_notes(&self) -> Result<Vec<Note>, Box<dyn Error>>;
    fn create_note(&self, note: CreateNoteEvent) -> Result<Note, Box<dyn Error>>;
    fn update_note_body(&self, event: UpdateNoteBodyEvent) -> Result<String, Box<dyn Error>>;
    fn update_note_bounds(&self, event: UpdateNoteBoundsEvent) -> Result<String, Box<dyn Error>>;
    fn delete_note_by_id(&self, id: i32) -> Result<(), Box<dyn Error>>;
}
