pub mod location;
pub mod window_size;

use gpui::{
    Bounds, Pixels, Point, SharedString, Size, TitlebarOptions, WindowBounds, WindowOptions,
};
use location::Location;
use window_size::WindowSize;

pub fn make_editor_option(title: &str, location: Location, size: WindowSize) -> WindowOptions {
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds {
            origin: Point::<Pixels>::from(location),
            size: Size::<Pixels>::from(size),
        })),
        titlebar: Some(TitlebarOptions {
            title: Some(SharedString::from(title.to_string())),
            ..Default::default()
        }),
        ..Default::default()
    }
}
