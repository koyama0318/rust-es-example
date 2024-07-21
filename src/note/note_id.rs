use crate::{note_error::NoteError, StoreId};
use uuid::Uuid;

#[derive(Clone)]
pub struct NoteId {
    id: String,
}

impl NoteId {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn default() -> Self {
        let id = Uuid::new_v4().to_string();
        NoteId { id }
    }

    pub fn new(id: String) -> Result<Self, NoteError> {
        match Uuid::parse_str(&id) {
            Ok(id) => Ok(Self { id: id.to_string() }),
            Err(_) => Err(NoteError::InvalidCommand),
        }
    }
}

impl StoreId for NoteId {
    fn key(&self) -> String {
        format!("NOTE#{}", self.id())
    }
}
