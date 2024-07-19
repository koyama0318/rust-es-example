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
}

fn handle(input: Input) {
    match input.command.as_str() {
        "create" => {
            let command = CreateNoteCommand {
                note: input.create_note,
            };
            let store_fn = create_store_fn();
            let workflow = create_note_workflow(store_fn);
            let result = workflow(command);

            println!("{:?}", result)
        }
        "complete" => {
            let command = CompletedNoteCommand {
                id: NoteId::default(),
            };
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
    };
    handle(input);
}
