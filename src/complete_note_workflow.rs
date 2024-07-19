use uuid::Uuid;

use crate::note::*;
use crate::AppError;
use crate::StoreFn;

pub struct CompleteNoteCommand {
    pub id: String,
}

#[derive(Debug)]
pub struct CompleteNotePayload {
    pub message: String,
}

pub trait CompleteNoteWorkFlow: Fn(CompleteNoteCommand) -> CompleteNotePayload {}
impl<T> CompleteNoteWorkFlow for T where T: Fn(CompleteNoteCommand) -> CompleteNotePayload {}

pub fn service(note: UnvalidatedNote) -> Result<CompletedNote, AppError> {
    validate(note).and_then(complete)
}

pub fn complete_note_workflow<F: StoreFn>(store_fn: F) -> impl CompleteNoteWorkFlow {
    move |cmd: CompleteNoteCommand| {
        let id = NoteId::new(cmd.id);

        let result =
            service.and_then(|(note, event)| store_fn(&note, &event).and_then(|_| Ok(note)));

        match result {
            Ok(note) => CompleteNotePayload {
                message: format!("id: {}, content: {:?}", note.id.id(), &note.content),
            },
            Err(e) => CompleteNotePayload {
                message: format!("Error: {:?}", e),
            },
        }
    }
}
