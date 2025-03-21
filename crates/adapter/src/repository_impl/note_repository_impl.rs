use std::error::Error;

use derive_new::new;
use kernel::{model::note::Note, repository::note_repository::NoteRepository};
use rusqlite::Connection;

#[derive(new)]
pub struct NoteRepositoryImpl {
    pub connection: Connection,
}

impl NoteRepository for NoteRepositoryImpl {
    fn get_notes(&self) -> Result<Vec<Note>, Box<dyn Error>> {
        let mut stmt = self.connection.prepare(
            "
            SELECT
              id
            , title
            , body
            , width
            , height
            , location_x
            , location_y
            , is_active
            FROM notes
            ORDER BY id DESC;
            ",
        )?;

        let notes = stmt
            .query_map([], |row| {
                Ok(Note {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    body: row.get(2)?,
                    width: row.get(3)?,
                    height: row.get(4)?,
                    location_x: row.get(5)?,
                    location_y: row.get(6)?,
                    is_active: row.get(7)?,
                })
            })?
            .map(|note| note.unwrap())
            .collect();
        Ok(notes)
    }

    fn create_note(&self, note: Note) -> Result<Note, Box<dyn Error>> {
        self.connection.execute(
            "
            insert into notes (id, title, body) values (?1, ?2, ?3)
            ",
            (&note.id, &note.title, &note.body),
        )?;
        Ok(note)
    }

    fn update_note_title(&self, note: Note) -> Result<Note, Box<dyn Error>> {
        self.connection.execute(
            "
            update notes set body = ?1 where id = ?2
            ",
            (&note.body, &note.id),
        )?;
        Ok(note)
    }

    fn update_note_body(&self, note: Note) -> Result<Note, Box<dyn Error>> {
        self.connection.execute(
            "
            update notes set title = ?1 where id = ?2
            ",
            (&note.title, &note.id),
        )?;
        Ok(note)
    }

    fn delete_note_by_id(&self, id: i32) -> Result<(), Box<dyn Error>> {
        self.connection.execute(
            "
            delete from notes where id = ?1
            ",
            (id,),
        )?;
        Ok(())
    }
}
