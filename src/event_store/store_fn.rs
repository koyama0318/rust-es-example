use super::*;
use crate::note::*;

pub trait StoreFn: Fn(&dyn StorableEvent) -> Result<(), NoteError> {}
impl<T> StoreFn for T where T: Fn(&dyn StorableEvent) -> Result<(), NoteError> {}

pub fn make_store_fn() -> impl StoreFn {
    |event| {
        // TODO: DB接続
        // let storable = event.into_store_event();
        // ctx.store(storable)
        Ok(())
    }
}
