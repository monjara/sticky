use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, InteractiveElement, KeyBinding,
    ParentElement, Pixels, Render, Styled, Window, WindowBounds, WindowOptions, actions, black,
    div, hsla,
};
use gpui_component::input::{InputEvent, TextInput};
use kernel::model::note::{UpdateNoteBodyEvent, UpdateNoteBoundsEvent};
use registry::{add_note, global_model::app_handler::AppHandler};

const CONTEXT: &str = "Editor";

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

actions!(
    editor,
    [
        NewEditor,
        CloseEditor,
        MoveWindowUp,
        MoveWindowDown,
        MoveWindowRight,
        MoveWindowLeft
    ]
);

pub fn init(cx: &mut App) {
    cx.bind_keys([
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-n", NewEditor, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-w", CloseEditor, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-k", MoveWindowUp, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-j", MoveWindowDown, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-l", MoveWindowRight, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-h", MoveWindowLeft, Some(CONTEXT)),
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
                .global::<AppHandler>()
                .note_handler()
                .get_by_id(id)
                .unwrap();

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

    fn close_editor(&mut self, _: &CloseEditor, window: &mut Window, cx: &mut Context<Self>) {
        cx.global::<AppHandler>()
            .note_handler()
            .toggle_note_active(&self.id.to_string());
        window.remove_window();
    }

    fn move_window_up(&mut self, _: &MoveWindowUp, window: &mut Window, cx: &mut Context<Self>) {
        self.move_winow(Direction::Up, window, cx);
    }

    fn move_window_down(
        &mut self,
        _: &MoveWindowDown,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.move_winow(Direction::Down, window, cx);
    }

    fn move_window_right(
        &mut self,
        _: &MoveWindowRight,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.move_winow(Direction::Right, window, cx);
    }

    fn move_window_left(
        &mut self,
        _: &MoveWindowLeft,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.move_winow(Direction::Left, window, cx);
    }

    fn move_winow(&self, direction: Direction, window: &mut Window, cx: &mut Context<Self>) {
        let mut bounds = window.bounds();
        match direction {
            Direction::Up => bounds.origin.y -= Pixels::from(20.),
            Direction::Down => bounds.origin.y += Pixels::from(20.),
            Direction::Right => bounds.origin.x += Pixels::from(20.),
            Direction::Left => bounds.origin.x -= Pixels::from(20.),
        }
        bounds.size = window.viewport_size();

        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        };
        window.remove_window();
        cx.open_window(options, |window, cx| Self::view(window, cx, &self.id))
            .unwrap();
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
            .on_action(cx.listener(Self::close_editor))
            .on_action(cx.listener(Self::move_window_up))
            .on_action(cx.listener(Self::move_window_down))
            .on_action(cx.listener(Self::move_window_right))
            .on_action(cx.listener(Self::move_window_left))
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
