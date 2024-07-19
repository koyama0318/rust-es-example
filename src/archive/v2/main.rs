mod es;
mod model;
mod store;

use es::*;
use model::*;
use store::*;

// MARK: - WorkFlow
trait CreateNoteWorkflow: Fn(Command) -> Result<(), AppError> {}
impl<T> CreateNoteWorkflow for T where T: Fn(Command) -> Result<(), AppError> {}

// TODO: receive eventStore closure
fn create_note_workflow() -> impl CreateNoteWorkflow {
    |command: Command| {
        let save_note_fn =
            |stateEvent: NoteStateEvent| -> Result<NoteState, DomainError> { Ok(stateEvent.state) };

        let replay_fn =
            || -> Result<NoteState, DomainError> { Ok(NoteState::UnCompleted(UncompletedNote {})) };

        // 1. replay aggregate from event store. If not found, create new aggregate.
        // 2. create note and event
        // 3. store event
        return replay_fn()
            .and_then(|state| mutate(state, create))
            .and_then(&save_note_fn)
            .map(|_| ())
            .map_err(|_| AppError::DomainError);
    }
}

// MARK: - Handler
enum Command {
    Create,
    Complete,
    Archive,
}

impl Command {
    fn from(input: Input) -> Result<Self, AppError> {
        match input.command.as_str() {
            "create" => Ok(Command::Create),
            "complete" => Ok(Command::Complete),
            "archive" => Ok(Command::Archive),
            _ => Err(AppError::InvalidCommand),
        }
    }
}

struct Input {
    command: String,
}

#[derive(Debug)]
struct Payload {
    code: i16,
}

impl Payload {
    fn from(result: Result<(), AppError>) -> Self {
        let code = if result.is_ok() { 200 } else { 500 };
        return Self { code };
    }
}

fn handle(input: Input) -> String {
    // Validate input and create command
    match Command::from(input) {
        Ok(command) => match command {
            Command::Create => {
                // make workflow
                let workflow = create_note_workflow();
                // call workflow
                let result = workflow(command);
                // make result to json string
                return format!("{:?}", Payload::from(result));
            }
            Command::Complete => unimplemented!(),
            Command::Archive => unimplemented!(),
        },
        Err(_) => String::from("Invalid command"),
    }
}

enum AppError {
    InvalidCommand,
    DomainError,
}

// MARK: - Main

fn main() {
    println!("Hello, world!");

    let input = Input {
        command: String::from("create"),
    };

    let result = handle(input);

    println!("Result: {:?}", result);
}
