pub mod repository;
mod utils;

use rusqlite::{Connection, Result};

pub struct Db {
    pub conn: Connection,
}

impl Db {
    pub fn new() -> Result<Self> {
        let conn = Self::connect()?;
        Ok(Self { conn })
    }

    pub fn connect() -> Result<Connection> {
        let conn = Connection::open("./database.sqlite")?;
        Ok(conn)
    }

    pub fn prepare_database(&self) -> Result<()> {
        self.conn.execute(
            "
            CREATE TABLE IF NOT EXISTS notes (
              id TEXT NOT NULL
            , title TEXT NOT NULL
            , body TEXT NOT NULL
            , is_active BOOLEAN NOT NULL
            );
            ",
            (),
        )?;
        Ok(())
    }
}
