use components::input::text_input::{InputEvent, TextInput};
use gpui::{App, AppContext, Context, Entity, ParentElement, Render, Styled, Window, div};
use registry::global_model::note_store::NoteStore;

pub struct Editor {
    input: Entity<TextInput>,
}

impl Editor {
    pub fn view(window: &mut Window, cx: &mut App, id: &str) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx, id))
    }

    pub fn new(window: &mut Window, cx: &mut Context<Self>, id: &str) -> Self {
        let input = cx.new(|cx| {
            let _note = cx
                .global::<NoteStore>()
                .note_accessor
                .get(id)
                .unwrap()
                .clone();

            TextInput::new(window, cx).multi_line()
            //.h_full()
            //.placeholder("Type here");
            //input.set_text(note.body, window, cx);
            //input
        });
        cx.subscribe_in(&input, window, Self::on_input).detach();

        Self { input }
    }

    pub fn on_input(
        &mut self,
        _: &Entity<TextInput>,
        event: &InputEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        match event {
            InputEvent::Change(text) => println!("{text}"),
            InputEvent::PressEnter => println!("PressEnter"),
            InputEvent::Focus => println!("Focus"),
            InputEvent::Blur => println!("Blur"),
        };
    }
}

impl Render for Editor {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<'_, Self>,
    ) -> impl gpui::IntoElement {
        div().w_full().h_full().child(self.input.clone())
    }
}
