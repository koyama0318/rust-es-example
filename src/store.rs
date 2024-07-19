use chrono::format;

use crate::error::AppError;
use crate::note::NoteId;
use crate::{note_state::*, UnvalidatedNote};

pub trait Storable {
    fn store(&self) -> Result<(), AppError>;
}

pub trait Event {
    fn make_event(&self) -> DomainEvent;
}

pub struct DomainEvent {
    pub aggregate_id: String,
    pub event_type: String,
    pub data: String,
}

fn get_events(key: String) -> Result<Vec<DomainEvent>, AppError> {
    Ok(vec![])
}

pub trait StoreFn: Fn(&dyn Storable, &dyn Event) -> Result<(), AppError> {}
impl<T> StoreFn for T where T: Fn(&dyn Storable, &dyn Event) -> Result<(), AppError> {}

pub trait GetAggregateFn: Fn(NoteId) -> Result<NoteState, AppError> {}
impl<T> GetAggregateFn for T where T: Fn(NoteId) -> Result<NoteState, AppError> {}

pub fn create_store_fn() -> impl StoreFn {
    |storable, event| Ok(())
}

pub fn get_aggregate_fn() -> impl GetAggregateFn {
    |id| {
        let key = format!("NOTE#{}", id.id());
        let events = get_events(key);

        events.and_then(|events| {
            events
                .into_iter()
                .try_fold(NoteState::default(), |state, event| {
                    let note_state = state.apply(event);
                    note_state
                })
        })
    }
}
