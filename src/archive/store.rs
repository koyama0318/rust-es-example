use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Event {
    id: String,
    aggregate_id: String,
    event_type: String,
    event_data: JSONData,
    timestamp: u64,
    version: u64,
}

#[derive(Debug, Clone)]
pub struct JSONData {
    json: String,
}

enum RepositoryError {
    InternalServer,
}

pub struct Repository {}

impl Repository {
    pub fn new() -> Self {
        Repository {}
    }
    pub fn store(&self, event: Event) -> Result<(), RepositoryError> {
        Ok(())
    }
}

// // presentation
// // persist event
// // event store
// // publish event
// // replay event

// // TODO: implements command_handler

// // command_handler example
// // 1. ui issues command
// // 2. handler replay events and make aggregate
// // 3. handler invokes methos on aggregate for handle command
// // 4. aggregate emits events
// // 5. the system appends events to event store

// // Event
// // - event_id
// // - aggregate_id
// // - event_type
// // - timestamp
// // - event_data
// // - verstion

// // Snapshot
// // - event_id
// // - aggregate_id
// // - version

// // event_stream
// // - snapthot: Option<SnapshotEvent>
// // - events: Vec<Event>
// // fn new(events) -> Self

// // EventStore
// // - fn get_event_stream(aggregate_id) -> event_stream<T> where T: Aggregate
// // - fn make_snapshot_event(event) -> SnapshotEvent<T> where: T: Aggregate
// // - fn publish_event(event) -> Result<(), Error>

// // SnapshotStore
// // fn get<T>(aggregate_id) -> Snapshot
// // fn save(Snapshot, version) -> Result<(), Error>

// // Protocol Aggregate
// // - fn replay_events<T>(snapshot, event_stream) -> aggregate where T: Aggregate

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Event<T> {
//     id: String,
//     aggregate_id: String,
//     event_type: String,
//     event_data: JSONData<T>,
//     timestamp: u64,
//     version: u64,
// }

// struct EventStream<T> {
//     snapshot: Option<Snapshot<T>>,
//     events: Vec<Event<T>>,
// }

// struct Snapshot<T> {
//     id: String,
//     aggregate_id: String,
//     data: JSONData<T>,
//     version: u64,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct JSONData<T> {
//     snapshot: T,
// }

// #[derive(Debug)]
// enum EventStoreError {
//     DatabaseError(String),
//     EventNotFound,
//     SnapshotError,
// }

// trait EventStore {
//     fn get_events(&self, aggregate_id: String) -> Result<EventStream, EventStoreError>;
//     fn publish(&self, event: Event) -> Result<(), EventStoreError>;
//     fn make_snapshot(&self, event: Event) -> Result<Snapshot, EventStoreError>;
// }

// trait Aggregate: Sized {
//     fn from(event_stream: EventStream) -> Self;
// }

// use chrono::{DateTime, Utc};

// pub enum TodoEvent {
//     Created(String),
//     TitleChanged(String),
//     Completed,
//     UnCompleted,
//     Deleted,
//     Restored,
// }

// // states
// #[derive(Debug, Clone)]
// pub struct UnvalidatedTodo {
//     title: String,
// }

// #[derive(Debug, Clone)]
// pub struct CreatedTodo {
//     id: String,
//     title: String,
//     is_done: bool,
//     updated_at: DateTime<Utc>,
// }

// impl Changable for CreatedTodo {
//     fn get_id(&self) -> String {
//         self.id.clone()
//     }
// }

// #[derive(Debug, Clone)]
// pub struct DeletedTodo {
//     id: String,
// }

// impl Changable for CreatedTodo {
//     fn get_id(&self) -> String {
//         self.id.clone()
//     }
// }

// // group
// trait Changable {
//     fn get_id(&self) -> String;
// }

// trait Todo {
//     fn replay(&self, event: TodoEvent) -> Self {
//         match event {
//             TodoEvent::Created(title) => Self {
//                 id: String::new(),
//                 title: title,
//                 updated_at: Utc::now(),
//             },
//             TodoEvent::TitleChanged(title) => Self {
//                 title,
//                 ..self.clone()
//             },
//             _ => Self { ..self.clone() },
//         }
//     }
// }

// T<State1> -> Event -> T<State2>
// replay(State1, Event) -> State2

// action(State1, Event) -> State2

// EventData(EventType, EventParam)
// fn store(ctx) -> impl FnMut(EventData) -> Result<> { store event data. }

// Store.get_events(id) -> Vec<EventData>

// Aggregate.replay(events) -> State
