use gpui::{Bounds, Pixels};

#[derive(Clone, Debug)]
pub struct Note {
    pub id: String,
    pub body: String,
    pub width: f32,
    pub height: f32,
    pub location_x: f32,
    pub location_y: f32,
    pub is_active: bool,
}

#[derive(Clone, Debug)]
pub struct UpdateNoteBodyEvent {
    pub id: String,
    pub body: String,
}

#[derive(Clone, Debug)]
pub struct UpdateNoteBoundsEvent {
    pub id: String,
    pub bounds: Bounds<Pixels>,
}

#[derive(Clone, Debug)]
pub struct UpdateNoteActiveEvent {
    pub id: String,
    pub is_active: bool,
}
