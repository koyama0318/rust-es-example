pub trait StorableEvent {
    fn from(event: StoreEvent) -> Self
    where
        Self: Sized;
    fn into_store_event(&self) -> StoreEvent;
}

pub struct StoreEvent {
    pub aggregate_id: String,
    pub event_type: String,
    pub data: String,
}
