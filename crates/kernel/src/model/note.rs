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

#[derive(Clone, Debug)]
pub struct CreateNoteEvent {
    pub id: String,
    pub title: String,
    pub body: String,
    pub width: f32,
    pub height: f32,
    pub location_x: f32,
    pub location_y: f32,
    pub is_active: bool,
}

impl From<CreateNoteEvent> for Note {
    fn from(note: CreateNoteEvent) -> Self {
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

#[derive(Clone, Debug)]
pub struct UpdateNoteTitleEvent {
    pub id: String,
    pub title: String,
}

#[derive(Clone, Debug)]
pub struct UpdateNoteBodyEvent {
    pub id: String,
    pub body: String,
}
