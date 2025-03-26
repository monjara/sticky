use std::error::Error;

use db::utils::gen_id;
use derive_new::new;
use kernel::{
    model::note::{Note, UpdateNoteActiveEvent, UpdateNoteBodyEvent, UpdateNoteBoundsEvent},
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
                    body: row.get(1)?,
                    width: row.get(2)?,
                    height: row.get(3)?,
                    location_x: row.get(4)?,
                    location_y: row.get(5)?,
                    is_active: row.get(6)?,
                })
            })?
            .map(|note| note.unwrap())
            .collect();
        Ok(notes)
    }

    fn get_note_by_id(&self, id: &str) -> Result<Option<Note>, Box<dyn Error>> {
        let mut stmt = self.connection.prepare(
            "
            SELECT
              id
            , body
            , width
            , height
            , location_x
            , location_y
            , is_active
            FROM notes
            WHERE id = ?1
            ",
        )?;

        let note = stmt
            .query_map([id], |row| {
                Ok(Note {
                    id: row.get(0)?,
                    body: row.get(1)?,
                    width: row.get(2)?,
                    height: row.get(3)?,
                    location_x: row.get(4)?,
                    location_y: row.get(5)?,
                    is_active: row.get(6)?,
                })
            })?
            .find(|note| note.is_ok())
            .unwrap();

        if let Ok(note) = note {
            Ok(Some(note))
        } else {
            Ok(None)
        }
    }

    fn create_note(&self) -> Result<Note, Box<dyn Error>> {
        let id = gen_id();
        let _ = self.connection.execute(
            "
            insert into notes (
              id
            , body
            , is_active
            , width
            , height
            , location_x
            , location_y
            ) values ($1, $2, $3, $4, $5, $6, $7)
            ",
            (&id, "", true, 200, 200, 200, 200),
        )?;
        let note = self.get_note_by_id(&id)?.unwrap();
        Ok(note)
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

    fn update_note_active(&self, event: UpdateNoteActiveEvent) -> Result<(), Box<dyn Error>> {
        self.connection.execute(
            "
            update notes set is_active = ?1 where id = ?2
            ",
            (&f32::from(event.is_active), &event.id),
        )?;
        Ok(())
    }

    fn delete_note_by_id(&self, id: &str) -> Result<(), Box<dyn Error>> {
        self.connection.execute(
            "
            delete from notes where id = ?1
            ",
            (id,),
        )?;
        Ok(())
    }
}
