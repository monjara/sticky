use std::error::Error;

use crate::model::note::Note;

pub trait NoteRepository {
    fn get_notes(&self) -> Result<Vec<Note>, Box<dyn Error>>;
    fn create_note(&self, note: Note) -> Result<Note, Box<dyn Error>>;
    fn update_note(&self, note: Note) -> Result<Note, Box<dyn Error>>;
    fn delete_note_by_id(&self, id: i32) -> Result<(), Box<dyn Error>>;
}
