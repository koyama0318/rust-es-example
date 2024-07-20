use crate::note::*;
use crate::note_state::unwrap_to_uncompleted;
use crate::GetAggregateFn;
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

pub fn complete_note_workflow<F1: StoreFn, F2: GetAggregateFn>(
    store_fn: F1,
    aggregate_fn: F2,
) -> impl CompleteNoteWorkFlow {
    move |cmd: CompleteNoteCommand| {
        let result = NoteId::new(cmd.id)
            .and_then(aggregate_fn)
            .and_then(unwrap_to_uncompleted)
            .and_then(complete)
            .and_then(|(note, event)| store_fn(&note, &event).and_then(|_| Ok(note)));

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
