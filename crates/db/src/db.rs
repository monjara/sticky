use rusqlite::{Connection, Result};
use ulid::Ulid;

#[derive(Debug)]
struct Note {
    id: String,
    title: String,
    body: String,
}

pub fn init() -> Result<()> {
    let path = "./database.sqlite";
    let conn = Connection::open(path)?;

    conn.execute(
        "
            CREATE TABLE IF NOT EXISTS notes (
              id TEXT NOT NULL
            , title TEXT NOT NULL
            , body TEXT NOT NULL
            );
        ",
        (),
    )?;

    let note = Note {
        id: Ulid::new().to_string(),
        title: "first note".to_string(),
        body: r"
            asdfasdfa
            asdfasdfa
            asdfasdfa
            asdfasdfa
            asdfasdfa


            asdfasdfa
            asdfasdfa
            "
        .to_string(),
    };

    conn.execute(
        "
        insert into notes (id, title, body) values (?1, ?2, ?3);
        ",
        (&note.id, &note.title, &note.body),
    )?;

    let note = Note {
        id: Ulid::new().to_string(),
        title: "second note".to_string(),
        body: r"
            asdfasdfa
            asdfasdfa
            asdfasdfa
            asdfasdfa
            asdfasdfa


            asdfasdfa
            asdfasdfa
            "
        .to_string(),
    };

    conn.execute(
        "
        insert into notes (id, title, body) values (?1, ?2, ?3);
        ",
        (&note.id, &note.title, &note.body),
    )?;

    let note = Note {
        id: Ulid::new().to_string(),
        title: "third note".to_string(),
        body: r"
            asdfasdfa
            asdfasdfa
            asdfasdfa
            asdfasdfa
            asdfasdfa


            asdfasdfa
            asdfasdfa
            "
        .to_string(),
    };

    conn.execute(
        "
        insert into notes (id, title, body) values (?1, ?2, ?3);
        ",
        (&note.id, &note.title, &note.body),
    )?;

    let mut stmt = conn.prepare("select id, title, body from notes order by id desc")?;

    let notes = stmt.query_map([], |row| {
        Ok(Note {
            id: row.get(0)?,
            title: row.get(1)?,
            body: row.get(2)?,
        })
    })?;

    for note in notes {
        println!("{note:?}");
    }

    Ok(())
}
