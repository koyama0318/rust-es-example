use serde::Serialize;
use serde_json::json;

use crate::store::DomainEvent;
use crate::Event;

#[derive(Serialize)]
pub struct NoteCreatedEvent {
    pub id: String,
    pub content: String,
}

pub enum NoteEvent {
    Created { id: String, data: NoteCreatedEvent },
    Completed { id: String },
}

impl Event for NoteEvent {
    fn make_event(&self) -> DomainEvent {
        match self {
            NoteEvent::Created { id, data } => DomainEvent {
                aggregate_id: format!("NOTE#{}", id),
                event_type: "created".to_string(),
                data: json!(data).to_string(),
            },
            NoteEvent::Completed { id } => DomainEvent {
                aggregate_id: format!("NOTE#{}", id),
                event_type: "completed".to_string(),
                data: "".to_string(),
            },
        }
    }
}
