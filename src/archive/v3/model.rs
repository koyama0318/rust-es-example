use crate::note_state::*;

pub struct NoteEvent {}

#[derive(Clone)]
pub struct UnvalidatedNote {}

#[derive(Clone)]
pub struct CompletedNote {}

pub fn validate(note: UnvalidatedNote) -> Result<(CompletedNote, NoteEvent), AppError> {
    Ok((CompletedNote {}, NoteEvent {}))
}

pub enum AppError {
    InvalidCommand,
    DomainError,
}
