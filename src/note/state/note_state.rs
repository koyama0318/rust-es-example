use crate::{note::*, State};

pub enum NoteState {
    Unvalidated(UnvalidatedNote),
    Uncompleted(UncompletedNote),
    Completed(CompletedNote),
}

impl State<NoteEvent, NoteError> for NoteState {
    fn default() -> NoteState {
        NoteState::Unvalidated(UnvalidatedNote::default())
    }

    fn apply(self, event: NoteEvent) -> Result<NoteState, NoteError> {
        match (self, event) {
            (NoteState::Unvalidated(note), NoteEvent::Created { id: _, data }) => {
                Ok(note.set(data.content))
                    .and_then(validate_and_create)
                    .and_then(|(note, _)| Ok(NoteState::Uncompleted(note)))
            }
            (NoteState::Uncompleted(note), NoteEvent::Completed { .. }) => Ok(note)
                .and_then(complete)
                .and_then(|(note, _)| Ok(NoteState::Completed(note))),
            _ => Err(NoteError::InvalidCommand),
        }
    }
}

pub fn unwrap_to_uncompleted(state: NoteState) -> Result<UncompletedNote, NoteError> {
    match state {
        NoteState::Uncompleted(note) => Ok(note),
        _ => Err(NoteError::InvalidCommand),
    }
}

pub fn validate_and_create(
    note: UnvalidatedNote,
) -> Result<(UncompletedNote, NoteEvent), NoteError> {
    validate(note).and_then(create)
}
