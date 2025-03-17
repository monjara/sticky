use gpui::{App, AppContext, Context, Entity, ParentElement, Render, Styled, Window, div};
use gpui_component::input::TextInput;

pub struct Editor {
    input: Entity<TextInput>,
}

impl Editor {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            TextInput::new(window, cx)
                .multi_line()
                .h_full()
                .placeholder("Type here")
        });

        Self { input }
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
