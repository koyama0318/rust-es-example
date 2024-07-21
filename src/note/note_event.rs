use serde::Serialize;
use serde_json::json;

use crate::event_store::*;
use crate::note::*;

#[derive(Serialize)]
pub struct NoteCreatedEvent {
    pub content: String,
}

pub enum NoteEvent {
    Created { id: NoteId, data: NoteCreatedEvent },
    Completed { id: NoteId },
}

impl StorableEvent for NoteEvent {
    fn from(event: StoreEvent) -> Self {
        NoteEvent::Created {
            id: NoteId::new(event.aggregate_id).unwrap(),
            data: NoteCreatedEvent {
                content: "".to_string(),
            },
        }
    }

    fn into_store_event(&self) -> StoreEvent {
        match self {
            NoteEvent::Created { id, data } => StoreEvent {
                aggregate_id: format!("NOTE#{}", id.id()),
                event_type: "created".to_string(),
                data: json!(data).to_string(),
            },
            NoteEvent::Completed { id } => StoreEvent {
                aggregate_id: format!("NOTE#{}", id.id()),
                event_type: "completed".to_string(),
                data: "".to_string(),
            },
        }
    }
}
