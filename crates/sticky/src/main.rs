mod view;

use gpui::{App, AppContext, Application, KeyBinding, WindowOptions};
use view::text_input::{
    Backspace, Copy, Cut, Delete, End, Home, InputExample, Left, Paste, Right, SelectAll,
    SelectLeft, SelectRight, ShowCharacterPalette, TextInput,
};
use view::text_wrap::TextWrap;

fn main() {
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);

        cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_cx| TextWrap {}))
            .unwrap();

        cx.bind_keys([
            KeyBinding::new("backspace", Backspace, None),
            KeyBinding::new("delete", Delete, None),
            KeyBinding::new("left", Left, None),
            KeyBinding::new("right", Right, None),
            KeyBinding::new("shift-left", SelectLeft, None),
            KeyBinding::new("shift-right", SelectRight, None),
            KeyBinding::new("cmd-a", SelectAll, None),
            KeyBinding::new("cmd-v", Paste, None),
            KeyBinding::new("cmd-c", Copy, None),
            KeyBinding::new("cmd-x", Cut, None),
            KeyBinding::new("home", Home, None),
            KeyBinding::new("end", End, None),
            KeyBinding::new("ctrl-cmd-space", ShowCharacterPalette, None),
        ]);

        cx.open_window(WindowOptions::default(), |_, cx| {
            let text_input = cx.new(|cx| TextInput {
                focus_handle: cx.focus_handle(),
                content: "".into(),
                placeholder: "Type here...".into(),
                selected_range: 0..0,
                selection_reversed: false,
                marked_range: None,
                last_layout: None,
                last_bounds: None,
                is_selecting: false,
            });
            cx.new(|cx| InputExample {
                text_input,
                recent_keystrokes: vec![],
                focus_handle: cx.focus_handle(),
            })
        })
        .unwrap();
    });
}
