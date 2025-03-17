use db::{Db, repository::note::NoteRepository};
use editor::editor::{Editor, window_options};
use gpui::{App, Application};

fn main() {
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);

        let db = match Db::new() {
            Ok(client) => {
                client.prepare_database().unwrap();
                client
            }
            Err(e) => panic!("Failed to connect to database: {e}"),
        };

        let notes = NoteRepository::new(db.conn)
            .select_all()
            .unwrap_or_else(|_| vec![]);

        if notes.is_empty() {
            cx.open_window(window_options(), Editor::view).unwrap();
        }

        for _note in notes {
            cx.open_window(window_options(), Editor::view).unwrap();
        }
    });
}
