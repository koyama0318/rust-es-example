use crate::{note::*, note_event::NoteEvent, AppError};

pub enum NoteState {
    Unvalidated(UnvalidatedNote),
    Uncompleted(UncompletedNote),
    Completed(CompletedNote),
}

impl NoteState {
    pub fn default() -> NoteState {
        NoteState::Unvalidated(UnvalidatedNote::default())
    }

    pub fn apply(self, event: NoteEvent) -> Result<NoteState, AppError> {
        match (self, event) {
            (NoteState::Unvalidated(note), NoteEvent::Created { id: _, data }) => {
                Ok(note.set(data.content))
                    .and_then(validate_and_create)
                    .and_then(|(note, _)| Ok(NoteState::Uncompleted(note)))
            }
            (NoteState::Uncompleted(note), NoteEvent::Completed { .. }) => Ok(note)
                .and_then(complete)
                .and_then(|(note, _)| Ok(NoteState::Completed(note))),
            _ => Err(AppError::InvalidCommand),
        }
    }
}

pub fn unwrap_to_uncompleted(state: NoteState) -> Result<UncompletedNote, AppError> {
    match state {
        NoteState::Uncompleted(note) => Ok(note),
        _ => Err(AppError::InvalidCommand),
    }
}

pub fn validate_and_create(
    note: UnvalidatedNote,
) -> Result<(UncompletedNote, NoteEvent), AppError> {
    validate(note).and_then(create)
}
