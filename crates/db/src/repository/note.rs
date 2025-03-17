use rusqlite::{Connection, Result};

use crate::utils::gen_id;

pub struct Note {
    id: String,
    title: String,
    body: String,
}

impl Default for Note {
    fn default() -> Self {
        Self {
            id: gen_id(),
            title: String::new(),
            body: String::new(),
        }
    }
}

pub struct NoteRepository {
    conn: Connection,
}

impl NoteRepository {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    pub fn select_all(&self) -> Result<Vec<Note>> {
        let mut stmt = self.conn.prepare(
            "
            select id, title, body from notes order by id desc
            ",
        )?;
        let notes = stmt
            .query_map([], |row| {
                Ok(Note {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    body: row.get(2)?,
                })
            })?
            .map(|note| note.unwrap())
            .collect();
        Ok(notes)
    }

    pub fn create(&self, note: &Note) -> Result<()> {
        self.conn.execute(
            "
            insert into notes (id, title, body) values (?1, ?2, ?3)
            ",
            (&note.id, &note.title, &note.body),
        )?;
        Ok(())
    }

    pub fn update(&self, note: &Note) -> Result<()> {
        self.conn.execute(
            "
            update notes set title = ?1, body = ?2 where id = ?3
            ",
            (&note.title, &note.body, &note.id),
        )?;
        Ok(())
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        self.conn.execute(
            "
            delete from notes where id = ?1
            ",
            (&id,),
        )?;
        Ok(())
    }
}
