#[derive(Clone, Debug)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub body: String,
    pub width: f32,
    pub height: f32,
    pub location_x: f32,
    pub location_y: f32,
    pub is_active: bool,
}

impl Note {
    pub fn new(
        id: String,
        title: String,
        body: String,
        width: f32,
        height: f32,
        location_x: f32,
        location_y: f32,
        is_active: bool,
    ) -> Self {
        Self {
            id,
            title,
            body,
            width,
            height,
            location_x,
            location_y,
            is_active,
        }
    }
}

impl From<kernel::model::note::Note> for Note {
    fn from(note: kernel::model::note::Note) -> Self {
        Self {
            id: note.id,
            title: note.title,
            body: note.body,
            width: note.width,
            height: note.height,
            location_x: note.location_x,
            location_y: note.location_y,
            is_active: note.is_active,
        }
    }
}
