use model::*;
mod model;
use note_state::*;
mod note_state;

fn create_note_workflow() -> impl Fn(NoteCommand) -> Result<(), AppError> {
    |command| {
        let store = NoteEventStore::new();
        let state = store.replay()?;
        let action = command.to_action();
        let (new_state, event) = NoteReducer::reduce(state, action);
        store.save(event)?;
        Ok(())
    }
}

fn main() {
    let cmd = NoteCommand::Create;
    let workflow = create_note_workflow();
    workflow(cmd);
    println!("Hello, world!");
}
