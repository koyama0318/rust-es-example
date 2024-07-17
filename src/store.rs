use crate::model::*;

pub trait EventStore {
    fn new() -> Self;
    fn save(&self, event: NoteStateEvent) -> Result<(), DomainError>;
    fn replay(&self) -> Result<NoteState, DomainError>;
}

pub struct NoteEventStore {}
impl EventStore for NoteEventStore {
    fn new() -> Self {
        Self {}
    }

    fn save(&self, event: NoteStateEvent) -> Result<(), DomainError> {
        Ok(())
    }

    fn replay(&self) -> Result<NoteState, DomainError> {
        Ok(NoteState::Unvalidated(UnvalidatedNote {}))
    }
}
