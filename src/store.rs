use crate::model::*;

pub struct NoteEventStore {}
impl NoteEventStore {
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
