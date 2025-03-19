use kernel::model;

#[derive(Clone, Debug)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub body: String,
    pub is_active: bool,
}

impl From<model::note::Note> for Note {
    fn from(note: model::note::Note) -> Self {
        Self {
            id: note.id,
            title: note.title,
            body: note.body,
            is_active: note.is_active,
        }
    }
}
