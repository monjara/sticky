use std::rc::Rc;

use adapter::repository_impl::note_repository_impl::NoteRepositoryImpl;
use gpui::Global;
use handler::note_handler::NoteHandler;
use rusqlite::Connection;

#[derive(Clone)]
pub struct AppRegistryImpl {
    pub note_handler: NoteHandler,
}

impl AppRegistryImpl {
    pub fn new(conn: Connection) -> Self {
        let note_handler = NoteHandler::new(Rc::new(NoteRepositoryImpl::new(conn)));

        Self { note_handler }
    }

    pub fn note_handler(&self) -> NoteHandler {
        self.note_handler.clone()
    }
}

impl Global for AppRegistryImpl {}
