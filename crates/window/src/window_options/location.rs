use gpui::{Pixels, Point};

pub struct Location {
    pub x: f32,
    pub y: f32,
}

impl Location {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<Location> for Point<Pixels> {
    fn from(location: Location) -> Self {
        Point::new(Pixels(location.x), Pixels(location.y))
    }
}
