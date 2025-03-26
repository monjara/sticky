use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, InteractiveElement, KeyBinding,
    ParentElement, Render, Styled, Window, actions, black, div, hsla,
};
use gpui_component::input::{InputEvent, TextInput};
use kernel::model::note::{UpdateNoteBodyEvent, UpdateNoteBoundsEvent};
use registry::{
    add_note,
    global_model::{app_handler::AppHandler, note_store::NoteStore},
};

const CONTEXT: &str = "Editor";

actions!(editor, [NewEditor, DeleteEditor]);

pub fn init(cx: &mut App) {
    cx.bind_keys([
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-n", NewEditor, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-shift-n", DeleteEditor, Some(CONTEXT)),
    ]);
}

pub struct Editor {
    id: String,
    focus_handle: FocusHandle,
    input: Entity<TextInput>,
}

impl Editor {
    pub fn view(window: &mut Window, cx: &mut App, id: &str) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx, id))
    }

    pub fn new(window: &mut Window, cx: &mut Context<Self>, id: &str) -> Self {
        let focus_handle = cx.focus_handle();
        cx.observe_window_bounds(window, |this, window, cx| {
            let bounds = window.bounds();
            cx.global::<AppHandler>()
                .note_handler()
                .update_note_bounds(UpdateNoteBoundsEvent {
                    id: this.id.clone(),
                    bounds,
                });
        })
        .detach();

        let input = cx.new(|cx| {
            let note = cx
                .global::<NoteStore>()
                .note_accessor
                .get(id)
                .unwrap()
                .clone();

            let mut input = TextInput::new(window, cx)
                .multi_line()
                .h_full()
                .appearance(false);
            input.set_text(note.body, window, cx);
            input.focus(window, cx);
            input
        });
        cx.subscribe_in(&input, window, Self::on_input).detach();

        Self {
            id: id.to_string(),
            focus_handle,
            input,
        }
    }

    pub fn on_input(
        &mut self,
        _: &Entity<TextInput>,
        event: &InputEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let InputEvent::Change(text) = event {
            cx.global::<AppHandler>()
                .note_handler()
                .update_note_body(UpdateNoteBodyEvent {
                    id: self.id.to_string(),
                    body: text.to_string(),
                });
        };
    }

    fn new_editor(&mut self, _: &NewEditor, _window: &mut Window, cx: &mut Context<Self>) {
        let note = cx.global::<AppHandler>().note_handler().create_note();
        add_note(cx, note);
    }

    fn delete_editor(&mut self, _: &DeleteEditor, _window: &mut Window, cx: &mut Context<Self>) {
        cx.global::<AppHandler>()
            .note_handler()
            .delete_note(&self.id);
    }
}

impl Focusable for Editor {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Editor {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<'_, Self>,
    ) -> impl gpui::IntoElement {
        div()
            .key_context(CONTEXT)
            .track_focus(&self.focus_handle.clone())
            .on_action(cx.listener(Self::new_editor))
            .on_action(cx.listener(Self::delete_editor))
            .bg(hsla(0.15, 0.96, 0.75, 1.))
            .text_color(black())
            .text_decoration_color(black())
            .opacity(1.)
            .w_full()
            .h_full()
            .items_center()
            .justify_center()
            .child(self.input.clone())
    }
}
