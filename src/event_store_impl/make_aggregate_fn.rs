use crate::event_store::*;
use crate::note::*;

fn get_events(key: String) -> Result<Vec<StoreEvent>, NoteError> {
    // TODO: DB接続
    // let events = ctx.getById(key)
    // Ok(events)
    Ok(vec![])
}

pub fn make_get_note_fn() -> impl GetNoteFn {
    |id: NoteId| {
        get_events(id.key()).and_then(|events| {
            events
                .into_iter()
                .map(StorableEvent::from)
                .try_fold(NoteState::default(), |state, event| state.apply(event))
        })
    }
}
