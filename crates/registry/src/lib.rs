pub mod global_model;

use db::Db;
use global_model::{app_handler::AppHandler, note_store::NoteStore};
use gpui::App;
use handler::model::note::Note;

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

pub fn init_store(cx: &mut App) {
    let notes = cx.global::<AppHandler>().note_handler.get_all();

    let note_store = NoteStore {
        notes,
        new_notes: vec![],
    };

    cx.set_global(note_store);
}

pub fn add_note(cx: &mut App, note: Note) {
    cx.global_mut::<NoteStore>().new_notes.push(note);
}
