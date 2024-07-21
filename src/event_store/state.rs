use super::*;

pub trait State<Event: StorableEvent, Error> {
    fn default() -> Self;
    fn apply(self, event: Event) -> Result<Self, Error>
    where
        Self: Sized;
}
