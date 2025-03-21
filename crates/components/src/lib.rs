use gpui::App;
use input::text_input;

pub mod input;
pub mod scroll;

pub fn init(cx: &mut App) {
    text_input::init(cx);
}
