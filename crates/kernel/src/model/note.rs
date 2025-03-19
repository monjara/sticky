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
