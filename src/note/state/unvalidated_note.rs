use crate::note::*;

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

pub fn validate(note: UnvalidatedNote) -> Result<ValidatedNote, NoteError> {
    let note = ValidatedNote {
        id: NoteId::default(),
        content: note.content,
    };
    Ok(note)
}

pub fn create(note: ValidatedNote) -> Result<(UncompletedNote, NoteEvent), NoteError> {
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
