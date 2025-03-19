use gpui::{App, Application};
use registry::global_model::note_store::NoteStore;
use window_editor::editor::{Editor, window_options};

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.activate(true);

        gpui_component::init(cx);
        registry::init(cx);

        let notes = cx.global::<NoteStore>().notes.clone();

        if notes.is_empty() {
            //cx.open_window(window_options(), |window, cx| Editor::view(window, cx, &id))
            //    .unwrap();
        }

        for note in notes {
            cx.open_window(window_options(), |window, cx| {
                Editor::view(window, cx, &note.id)
            })
            .unwrap();
        }
    });
}
