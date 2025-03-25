pub mod location;
pub mod window_size;

use gpui::{Bounds, Pixels, Point, Size, WindowBounds, WindowOptions};
use location::Location;
use window_size::WindowSize;

pub fn make_editor_option(location: Location, size: WindowSize) -> WindowOptions {
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds {
            origin: Point::<Pixels>::from(location),
            size: Size::<Pixels>::from(size),
        })),
        ..Default::default()
    }
}
