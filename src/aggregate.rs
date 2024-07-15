use serde::Serialize;
use serde_json::to_string;

pub trait Aggregate: Sized {
    fn replay(events: Vec<DomainEvent>) -> Self
    where
        Self: Default,
    {
        let mut aggregate = Self::default();
        for event in events {
            aggregate = aggregate.process(event);
        }
        aggregate
    }

    fn process(&mut self, event: DomainEvent) -> Self;

    fn get_aggregate_id(&self) -> String;
    fn make_snapshot(&self) -> String;
}

pub trait NoteAggregate: Aggregate + Serialize {
    fn get_aggregate_id(&self) -> String {
        format!("NOTE#{}", self.get_id())
    }

    fn make_snapshot(&self) -> String {
        to_string(self).unwrap_or_else(|_| String::from("{}"))
    }

    fn get_id(&self) -> String;
}

#[derive(Default, Serialize)]
pub struct UnvalidatedNote {
    // Note struct fields here
}

impl NoteAggregate for UnvalidatedNote {
    fn get_id(&self) -> String {
        // Return the id of the note
        String::from("note_id") // Placeholder implementation
    }
}

impl Aggregate for UnvalidatedNote {
    fn get_aggregate_id(&self) -> String {
        <Self as NoteAggregate>::get_aggregate_id(self)
    }

    fn process(&mut self, event: DomainEvent) -> Self {
        // Process the event and return the new state
        // This is a placeholder implementation
        UnvalidatedNote {}
    }

    fn make_snapshot(&self) -> String {
        to_string(self).unwrap_or_else(|_| String::from("{}"))
    }
}

pub struct DomainEvent {
    id: String,
    aggregate_id: String,
    event_type: String,
    event_data: String,
    timestamp: u64,
    version: u64,
}
