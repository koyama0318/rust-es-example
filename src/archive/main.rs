mod event;
mod model;
mod store;

use model::*;
use store::*;

// fn main() {
// let repo = Repository::new();

// let note = UnvalidatedNote::new("Hello", "Hello world. This is a note.");

// let (note_event, note) = create(note);
// let event = Event { id: String,
//     aggregate_id: String,
//     event_type: String,
//     event_data: JSONData,
//     timestamp: u64,
//     version: u64, }
// repo.store(event);
// 1. Empty Note created
// 2. Note itle changed
// 3. Note completed
// 4. Note uncompleted
// 5. Note deleted
// 6. Note restored
// 7. Note title changed
// 8. Note completed
// }
