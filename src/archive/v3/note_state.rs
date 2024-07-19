use crate::model::*;

pub enum NoteCommand {
    Create,
    Complete,
    Archive,
}

pub enum NoteState {
    Unvalidated(UnvalidatedNote),
    Completed(CompletedNote),
}
impl NoteState {
    fn reduce(self, command: NoteCommand) -> Result<(NoteState, NoteEvent), AppError> {
        match command {
            NoteCommand::Create => self.mutate(validate),
            NoteCommand::Complete => {
                unimplemented!();
            }
            NoteCommand::Archive => {
                unimplemented!();
            }
        }
    }

    fn mutate<T1, T2, Action>(self, action: Action) -> Result<(NoteState, NoteEvent), AppError>
    where
        T1: Note,
        T2: Note,
        Action: Fn(T1) -> Result<(T2, NoteEvent), AppError>,
    {
        T1::unwrap(self)
            .and_then(action)
            .and_then(|(note, event)| Ok((note.wrap(), event)))
    }
}

pub trait Note {
    fn wrap(&self) -> NoteState;
    fn unwrap(state: NoteState) -> Result<Self, AppError>
    where
        Self: Sized;
}

impl Note for UnvalidatedNote {
    fn wrap(&self) -> NoteState {
        NoteState::Unvalidated(self.clone())
    }
    fn unwrap(state: NoteState) -> Result<Self, AppError> {
        match state {
            NoteState::Unvalidated(note) => Ok(note),
            _ => Err(AppError::DomainError),
        }
    }
}

impl Note for CompletedNote {
    fn wrap(&self) -> NoteState {
        NoteState::Completed(self.clone())
    }
    fn unwrap(state: NoteState) -> Result<Self, AppError> {
        match state {
            NoteState::Completed(note) => Ok(note),
            _ => Err(AppError::DomainError),
        }
    }
}
