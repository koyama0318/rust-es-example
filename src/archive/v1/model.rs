use chrono::{DateTime, Utc};

use crate::event::{self, NoteEvent};

// MARK: - states
#[derive(Debug, Clone)]
pub struct UnvalidatedNote {
    title: String,
    body: String,
}

impl UnvalidatedNote {
    pub fn new(title: &str, body: &str) -> Self {
        UnvalidatedNote {
            title: title.to_string(),
            body: body.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UncompletedNote {
    id: String,
    title: String,
    body: String,
    updated_at: DateTime<Utc>,
}

impl Archivable for UncompletedNote {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn get_body(&self) -> String {
        self.body.clone()
    }

    fn get_updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    fn from_archived(note: ArchivedNote) -> Self {
        UncompletedNote {
            id: note.id,
            title: note.title,
            body: note.body,
            updated_at: note.updated_at,
        }
    }
}

pub struct ArchivedNote {
    id: String,
    title: String,
    body: String,
    updated_at: DateTime<Utc>,
}

pub struct CompletedNote {
    id: String,
    title: String,
    body: String,
    updated_at: DateTime<Utc>,
}

impl Archivable for CompletedNote {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn get_body(&self) -> String {
        self.body.clone()
    }

    fn get_updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    fn from_archived(note: ArchivedNote) -> Self {
        CompletedNote {
            id: note.id,
            title: note.title,
            body: note.body,
            updated_at: note.updated_at,
        }
    }
}

// MARK: - behaviors
pub trait Archivable {
    fn get_id(&self) -> String;
    fn get_title(&self) -> String;
    fn get_body(&self) -> String;
    fn get_updated_at(&self) -> DateTime<Utc>;

    fn from_archived(note: ArchivedNote) -> Self
    where
        Self: Sized;
}

// MARK: - actions
pub fn create(note: UnvalidatedNote) -> (NoteEvent, UncompletedNote) {
    let event = NoteEvent::NoteCreated(note.clone());
    let note = UncompletedNote {
        id: String::new(),
        title: String::new(),
        body: String::new(),
        updated_at: Utc::now(),
    };
    (event, note)
}

pub fn archive<T: Archivable>(note: T) -> (NoteEvent, ArchivedNote) {
    let note = ArchivedNote {
        id: note.get_id(),
        title: note.get_title(),
        body: note.get_body(),
        updated_at: note.get_updated_at(),
    };
    let event = NoteEvent::NoteArchived;
    (event, note)
}

pub fn restore<T: Archivable>(note: ArchivedNote) -> (NoteEvent, T) {
    (NoteEvent::NoteRestored, T::from_archived(note))
}

pub fn uncomplete(note: CompletedNote) -> (NoteEvent, UncompletedNote) {
    let note = UncompletedNote {
        id: note.id,
        title: note.title,
        body: note.body,
        updated_at: note.updated_at,
    };
    (NoteEvent::NoteCompleted, note)
}

pub fn complete(note: UncompletedNote) -> (NoteEvent, CompletedNote) {
    let note = CompletedNote {
        id: note.id,
        title: note.title,
        body: note.body,
        updated_at: note.updated_at,
    };
    (NoteEvent::NoteCompleted, note)
}
