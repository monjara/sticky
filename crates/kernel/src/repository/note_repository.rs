use std::error::Error;

use crate::model::note::{Note, UpdateNoteBodyEvent, UpdateNoteBoundsEvent};

pub trait NoteRepository {
    fn get_notes(&self) -> Result<Vec<Note>, Box<dyn Error>>;
    fn get_note_by_id(&self, id: &str) -> Result<Option<Note>, Box<dyn Error>>;
    fn create_note(&self) -> Result<Note, Box<dyn Error>>;
    fn update_note_body(&self, event: UpdateNoteBodyEvent) -> Result<String, Box<dyn Error>>;
    fn update_note_bounds(&self, event: UpdateNoteBoundsEvent) -> Result<String, Box<dyn Error>>;
    fn delete_note_by_id(&self, id: &str) -> Result<(), Box<dyn Error>>;
}
