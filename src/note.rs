use crate::{
    error::AppError,
    note_event::{NoteCreatedEvent, NoteEvent},
    Storable,
};
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

    pub fn new(id: String) -> Result<Self, AppError> {
        match Uuid::parse_str(&id) {
            Ok(id) => Ok(Self { id: id.to_string() }),
            Err(_) => Err(AppError::InvalidCommand),
        }
    }
}

pub struct UnvalidatedNote {
    pub content: String,
}

impl UnvalidatedNote {
    pub fn default() -> Self {
        UnvalidatedNote {
            content: "".to_string(),
        }
    }

    pub fn set(self, content: String) -> Self {
        UnvalidatedNote { content }
    }
}

pub struct ValidatedNote {
    pub id: NoteId,
    pub content: String,
}

pub struct UncompletedNote {
    pub id: NoteId,
    pub content: String,
}

impl Storable for UncompletedNote {
    fn store(&self) -> Result<(), AppError> {
        Ok(())
    }
}

pub struct CompletedNote {
    pub id: NoteId,
    pub content: String,
}

impl Storable for CompletedNote {
    fn store(&self) -> Result<(), AppError> {
        Ok(())
    }
}

pub fn validate(note: UnvalidatedNote) -> Result<ValidatedNote, AppError> {
    let note = ValidatedNote {
        id: NoteId::default(),
        content: note.content,
    };
    Ok(note)
}

pub fn create(note: ValidatedNote) -> Result<(UncompletedNote, NoteEvent), AppError> {
    let uncompleted = UncompletedNote {
        id: note.id.clone(),
        content: note.content.clone(),
    };
    let event = NoteEvent::Created {
        id: note.id,
        data: NoteCreatedEvent {
            content: note.content,
        },
    };
    Ok((uncompleted, event))
}

pub fn complete(note: UncompletedNote) -> Result<(CompletedNote, NoteEvent), AppError> {
    let completed = CompletedNote {
        id: note.id.clone(),
        content: note.content,
    };
    let event = NoteEvent::Completed { id: note.id };
    Ok((completed, event))
}
