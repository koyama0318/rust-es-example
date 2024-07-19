use crate::{note::*, note_event::NoteEvent, AppError};

pub enum NoteState {
    Unvalidated(UnvalidatedNote),
    Uncompleted(UncompletedNote),
}

impl NoteState {
    pub fn default() -> NoteState {
        NoteState::Unvalidated(UnvalidatedNote::default())
    }

    pub fn apply(self, event: NoteEvent) -> Result<NoteState, AppError> {
        match (self, event) {
            (NoteState::Unvalidated(note), NoteEvent::Created { id, data }) => {
                let note = note.set(data.content);
                let note = UnvalidatedNote::default();
                Ok(NoteState::Unvalidated(note))
            }
            (NoteState::Uncompleted(_), NoteEvent::Completed { .. }) => {
                let note = UnvalidatedNote::default();
                Ok(NoteState::Unvalidated(note))
            }
            _ => Err(AppError::InvalidCommand),
        }
    }
}
