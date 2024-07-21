use crate::note::*;

pub struct UncompletedNote {
    pub id: NoteId,
    pub content: String,
}

pub fn complete(note: UncompletedNote) -> Result<(CompletedNote, NoteEvent), NoteError> {
    let completed = CompletedNote {
        id: note.id.clone(),
        content: note.content,
    };
    let event = NoteEvent::Completed { id: note.id };
    Ok((completed, event))
}
