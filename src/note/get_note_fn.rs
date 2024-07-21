use super::*;
use crate::event_store::*;

pub trait GetNoteFn: GetAggregateFn<NoteId, NoteState, NoteError> {}
impl<F> GetNoteFn for F where F: GetAggregateFn<NoteId, NoteState, NoteError> {}
