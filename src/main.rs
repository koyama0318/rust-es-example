mod complete_note_workflow;
mod create_note_workflow;
mod error;
mod note;
mod note_event;
mod note_state;
mod store;

use complete_note_workflow::*;
use create_note_workflow::*;
use error::*;
use note::*;
use store::*;

struct Input {
    command: String,
    create_note: UnvalidatedNote,
    update_id: String,
}

fn handle(input: Input) {
    match input.command.as_str() {
        "create" => {
            let command = CreateNoteCommand {
                note: input.create_note,
            };
            let store_fn = make_store_fn();
            let workflow = create_note_workflow(store_fn);
            let payload = workflow(command);
            println!("{:?}", payload);
        }
        "complete" => {
            let command = CompleteNoteCommand {
                id: input.update_id,
            };
            let store_fn = make_store_fn();
            let aggregate_fn = make_aggregate_fn();
            let workflow = complete_note_workflow(store_fn, aggregate_fn);
            let payload = workflow(command);
            println!("{:?}", payload);
        }
        _ => {}
    }
}

fn main() {
    let input = Input {
        command: "create".to_string(),
        create_note: UnvalidatedNote {
            content: "New note contents".to_string(),
        },
        update_id: "".to_string(),
    };
    handle(input);
}
