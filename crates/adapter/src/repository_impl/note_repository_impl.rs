use std::error::Error;

use derive_new::new;
use kernel::{
    model::note::{
        CreateNoteEvent, Note, UpdateNoteBodyEvent, UpdateNoteBoundsEvent, UpdateNoteTitleEvent,
    },
    repository::note_repository::NoteRepository,
};
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

    fn create_note(&self, event: CreateNoteEvent) -> Result<Note, Box<dyn Error>> {
        let note = Note::from(event);

        self.connection.execute(
            "
            insert into notes (
              id
            , title
            , body
            , is_active
            , width
            , height
            , location_x
            , location_y
            ) values ($1, $2, $3, $4, $5, $6, $7, $8)
            ",
            (
                &note.id,
                &note.title,
                &note.body,
                &note.is_active,
                &note.width,
                &note.height,
                &note.location_x,
                &note.location_y,
            ),
        )?;
        Ok(note)
    }

    fn update_note_title(&self, event: UpdateNoteTitleEvent) -> Result<String, Box<dyn Error>> {
        self.connection.execute(
            "
            update notes set title = ?1 where id = ?2
            ",
            (&event.title, &event.id),
        )?;
        Ok(event.id)
    }

    fn update_note_body(&self, event: UpdateNoteBodyEvent) -> Result<String, Box<dyn Error>> {
        self.connection.execute(
            "
            update notes set body = ?1 where id = ?2
            ",
            (&event.body, &event.id),
        )?;
        Ok(event.id)
    }

    fn update_note_bounds(&self, event: UpdateNoteBoundsEvent) -> Result<String, Box<dyn Error>> {
        self.connection.execute(
            "
            update notes set
              width = ?1
            , height = ?2
            , location_x = ?3
            , location_y = ?4
            where id = ?5
            ",
            (
                &f32::from(event.bounds.size.width),
                &f32::from(event.bounds.size.height),
                &f32::from(event.bounds.origin.x),
                &f32::from(event.bounds.origin.y),
                &event.id,
            ),
        )?;
        Ok(event.id)
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
