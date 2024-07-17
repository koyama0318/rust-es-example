pub trait Note {
    fn from(state: NoteState) -> Result<Self, DomainError>
    where
        Self: Sized;
    fn to_state(&self) -> Result<NoteState, DomainError>;
}

// MARK: - Domain
#[derive(Debug, Clone)]
pub struct UnvalidatedNote {}
impl Note for UnvalidatedNote {
    fn to_state(&self) -> Result<NoteState, DomainError> {
        Ok(NoteState::Unvalidated(self.clone()))
    }

    fn from(state: NoteState) -> Result<Self, DomainError> {
        match state {
            NoteState::Unvalidated(note) => Ok(note),
            _ => Err(DomainError::ConvertFailed),
        }
    }
}

pub struct ValidatedNote {}

#[derive(Debug, Clone)]
pub struct UncompletedNote {}
impl Note for UncompletedNote {
    fn to_state(&self) -> Result<NoteState, DomainError> {
        Ok(NoteState::UnCompleted(self.clone()))
    }

    fn from(state: NoteState) -> Result<Self, DomainError> {
        match state {
            NoteState::UnCompleted(note) => Ok(note),
            _ => Err(DomainError::ConvertFailed),
        }
    }
}

// MAKR: - Actions
pub fn validate(note: UnvalidatedNote) -> Result<ValidatedNote, DomainError> {
    Ok(ValidatedNote {})
}

pub fn create(note: UnvalidatedNote) -> Result<(UncompletedNote, NoteEvent), DomainError> {
    validate(note).and_then(|_| Ok((UncompletedNote {}, NoteEvent::Completed)))
}

// MARK: State
#[derive(Debug, Clone)]
pub enum NoteState {
    Unvalidated(UnvalidatedNote),
    UnCompleted(UncompletedNote),
}

pub fn mutate<T1, T2, Action>(
    state: NoteState,
    action: Action,
) -> Result<NoteStateEvent, DomainError>
where
    T1: Note,
    T2: Note,
    Action: Fn(T1) -> Result<(T2, NoteEvent), DomainError>,
{
    T1::from(state.clone())
        .and_then(action)
        .and_then(|(note, event)| {
            note.to_state().map(|state| NoteStateEvent {
                state: state,
                event: event,
            })
        })
}

// MARK: Event
#[derive(Clone)]
pub enum NoteEvent {
    Completed,
}

// MARK: StateEvent
#[derive(Clone)]
pub struct NoteStateEvent {
    pub state: NoteState,
    pub event: NoteEvent,
}

// MARK: Error
pub enum DomainError {
    ConvertFailed,
}
