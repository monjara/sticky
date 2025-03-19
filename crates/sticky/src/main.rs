use db::Db;
use editor::editor::{Editor, window_options};
use gpui::{App, Application};
use registry::AppRegistryImpl;

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.activate(true);

        gpui_component::init(cx);

        let db = match Db::new() {
            Ok(client) => {
                client.prepare_database().unwrap();
                client
            }
            Err(e) => panic!("Failed to connect to database: {e}"),
        };

        cx.set_global(AppRegistryImpl::new(db.conn));

        let notes = cx
            .global::<AppRegistryImpl>()
            .note_respository()
            .get_notes()
            .unwrap();

        println!("notes: {notes:?}");

        if notes.is_empty() {
            cx.open_window(window_options(), Editor::view).unwrap();
        }

        for _note in notes {
            cx.open_window(window_options(), Editor::view).unwrap();
        }
    });
}
