use super::model::*;
use super::store::*;

pub enum NoteEvent {
    NoteCreated(UnvalidatedNote),
    NoteTitleChanged(String),
    NoteBodyChanged(String),
    NoteCompleted,
    NoteUncompleted,
    NoteArchived,
    NoteRestored,
}

trait NoteAggregate {
    fn get_aggregate_id(&self) -> String;
    fn replay<T: NoteAggregate>(events: Event) -> T;
}
