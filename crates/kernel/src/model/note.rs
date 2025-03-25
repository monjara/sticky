use gpui::{Bounds, Pixels, Point, Size};

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

impl From<CreateNoteEvent> for Note {
    fn from(note: CreateNoteEvent) -> Self {
        let Bounds {
            origin: Point { x, y },
            size: Size { width, height },
        } = note.bounds;

        Self {
            id: note.id,
            body: note.body,
            width,
            height,
            location_x: x,
            location_y: y,
            is_active: note.is_active,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CreateNoteEvent {
    pub id: String,
    pub body: String,
    pub bounds: Bounds<f32>,
    pub is_active: bool,
}

impl From<Note> for CreateNoteEvent {
    fn from(note: Note) -> Self {
        Self {
            id: note.id,
            body: note.body,
            bounds: Bounds {
                origin: Point {
                    x: note.location_x,
                    y: note.location_y,
                },
                size: Size {
                    width: note.width,
                    height: note.height,
                },
            },
            is_active: note.is_active,
        }
    }
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
