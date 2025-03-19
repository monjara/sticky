use std::rc::Rc;

use adapter::repository_impl::note_repository_impl::NoteRepositoryImpl;
use gpui::Global;
use kernel::repository::note_repository::NoteRepository;
use rusqlite::Connection;

#[derive(Clone)]
pub struct AppRegistryImpl {
    note_repository: Rc<dyn NoteRepository>,
}

impl AppRegistryImpl {
    pub fn new(conn: Connection) -> Self {
        let note_repository = Rc::new(NoteRepositoryImpl::new(conn));

        Self { note_repository }
    }

    pub fn note_respository(&self) -> Rc<dyn NoteRepository> {
        self.note_repository.clone()
    }
}

impl Global for AppRegistryImpl {}
