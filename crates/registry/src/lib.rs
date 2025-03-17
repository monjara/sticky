use std::sync::Arc;

use adapter::repository_impl::note_repository_impl::NoteRepositoryImpl;
use kernel::repository::note_repository::NoteRepository;
use rusqlite::Connection;

pub struct AppRegistryImpl {
    note_repository: Arc<dyn NoteRepository>,
}

impl AppRegistryImpl {
    pub fn new(conn: Connection) -> Self {
        let note_repository = Arc::new(NoteRepositoryImpl::new(conn));

        Self { note_repository }
    }

    pub fn note_respository(&self) -> Arc<dyn NoteRepository> {
        self.note_repository.clone()
    }
}
