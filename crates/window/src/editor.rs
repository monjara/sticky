use gpui::{
    App, AppContext, Context, Entity, ParentElement, Render, Styled, Window, black, div, hsla,
};
use gpui_component::input::{InputEvent, TextInput};
use kernel::model::note::{UpdateNoteBodyEvent, UpdateNoteBoundsEvent};
use registry::global_model::{app_handler::AppHandler, note_store::NoteStore};

pub struct Editor {
    id: String,
    input: Entity<TextInput>,
}

impl Editor {
    pub fn view(window: &mut Window, cx: &mut App, id: &str) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx, id))
    }

    pub fn new(window: &mut Window, cx: &mut Context<Self>, id: &str) -> Self {
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
            input
        });
        cx.subscribe_in(&input, window, Self::on_input).detach();

        Self {
            id: id.to_string(),
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
}

impl Render for Editor {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<'_, Self>,
    ) -> impl gpui::IntoElement {
        div()
            .bg(hsla(0.15, 0.96, 0.75, 1.))
            .text_color(black())
            .text_decoration_color(black())
            .opacity(1.)
            .w_full()
            .h_full()
            .items_center()
            .justify_center()
            //.pl_3()
            //.pr_2()
            //.py_2()
            .child(self.input.clone())
    }
}
