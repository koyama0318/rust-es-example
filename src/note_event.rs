use serde::Serialize;
use serde_json::json;

use crate::store::DomainEvent;
use crate::{Event, NoteId};

#[derive(Serialize)]
pub struct NoteCreatedEvent {
    pub content: String,
}

pub enum NoteEvent {
    Created { id: NoteId, data: NoteCreatedEvent },
    Completed { id: NoteId },
}

impl Event for NoteEvent {
    fn make_event(&self) -> DomainEvent {
        match self {
            NoteEvent::Created { id, data } => DomainEvent {
                aggregate_id: format!("NOTE#{}", id.id()),
                event_type: "created".to_string(),
                data: json!(data).to_string(),
            },
            NoteEvent::Completed { id } => DomainEvent {
                aggregate_id: format!("NOTE#{}", id.id()),
                event_type: "completed".to_string(),
                data: "".to_string(),
            },
        }
    }
}
