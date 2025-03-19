use gpui::{
    App, AppContext, Bounds, Context, Entity, ParentElement, Pixels, Point, Render, Size, Styled,
    Window, WindowBounds, WindowOptions, div,
};
use gpui_component::input::{InputEvent, TextInput};

pub struct Editor {
    input: Entity<TextInput>,
}

impl Editor {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            let mut input = TextInput::new(window, cx)
                .multi_line()
                .h_full()
                .placeholder("Type here");
            input.set_text("default_value", window, cx);
            input
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

pub fn window_options() -> gpui::WindowOptions {
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds {
            origin: Point::new(Pixels(100.0), Pixels(100.0)),
            size: Size::new(Pixels(300.0), Pixels(200.0)),
        })),
        ..Default::default()
    }
}
