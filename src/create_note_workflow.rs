use crate::note::*;
use crate::StoreFn;

pub struct CreateNoteCommand {
    pub note: UnvalidatedNote,
}

#[derive(Debug)]
pub struct CreateNotePayload {
    pub message: String,
}

pub trait CreateNoteWorkFlow: Fn(CreateNoteCommand) -> CreateNotePayload {}
impl<T> CreateNoteWorkFlow for T where T: Fn(CreateNoteCommand) -> CreateNotePayload {}

pub fn create_note_workflow<F: StoreFn>(store_fn: F) -> impl CreateNoteWorkFlow {
    move |cmd: CreateNoteCommand| {
        let result = validate(cmd.note)
            .and_then(create)
            .and_then(|(note, event)| store_fn(&note, &event).and_then(|_| Ok(note)));

        match result {
            Ok(note) => CreateNotePayload {
                message: format!("id: {}, content: {:?}", note.id.id(), &note.content),
            },
            Err(e) => CreateNotePayload {
                message: format!("Error: {:?}", e),
            },
        }
    }
}
