pub mod global_model;

use std::collections::HashMap;

use db::Db;
use global_model::{app_handler::AppHandler, note_store::NoteStore};
use gpui::App;

pub fn init(cx: &mut App) {
    init_handler(cx);
    init_store(cx);
}

fn init_handler(cx: &mut App) {
    let db = match Db::new() {
        Ok(client) => {
            client.prepare_database().unwrap();
            client
        }
        Err(e) => panic!("Failed to connect to database: {e}"),
    };
    cx.set_global(AppHandler::new(db.conn));
}

fn init_store(cx: &mut App) {
    let notes = cx.global::<AppHandler>().note_handler.get_all();

    let map = notes.iter().fold(HashMap::new(), |mut map, note| {
        map.insert(note.id.clone(), note.clone());
        map
    });

    let note_store = NoteStore {
        notes,
        note_accessor: map,
    };

    cx.set_global(note_store);
}
