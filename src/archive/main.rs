// use serde::Serialize;

// fn main() {
//     println!("hello")
// }

// struct DomainEvent {
//     id: String,
// }

// trait Aggregate {
//     fn aggregate_id(&self) -> String;
//     fn snapshot(&self) -> String;
//     // fn replay(events: Vec<DomainEvent>) -> Self;
// }

// struct UserAggregate {
//     user: User,
// }
// impl Aggregate for UserAggregate {
//     fn aggregate_id(&self) -> String {
//         match &self.user {
//             User::UnvalidatedUser(unvalidated_user) => format!("USER#{}", unvalidated_user.name),
//             User::RegisteredUser(registered_user) => format!("USER#{}", registered_user.id.id),
//         }
//     }

//     fn snapshot(&self) -> String {
//         serde_json::to_string(&self.user).unwrap()
//     }

//     // fn replay(events: Vec<DomainEvent>) -> Self {
//     //     let mut user: User = User::UnvalidatedUser(UnvalidatedUser {
//     //         name: String::new(),
//     //     });

//     //     for event in events {
//     //         match UserEvent::from(event) {
//     //             UserEvent::UserRegistered => {
//     //                 user = User::RegisteredUser(validate(user));
//     //             }
//     //             UserEvent::UserCancelled => {}
//     //         }
//     //     }
//     //     return UserAggregate { user: user };
//     // }
// }

// enum UserEvent {
//     UserRegistered,
//     UserCancelled,
// }
// impl UserEvent {
//     fn from(event: DomainEvent) -> UserEvent {
//         UserEvent::UserCancelled
//     }
// }

// #[derive(Serialize)]
// enum User {
//     UnvalidatedUser(UnvalidatedUser),
//     RegisteredUser(RegisteredUser),
// }

// trait Usera {}

// #[derive(Serialize, Clone)]
// struct UserId {
//     id: String,
// }

// #[derive(Serialize, Clone)]
// struct UnvalidatedUser {
//     name: String,
// }

// impl Usera for UnvalidatedUser {}

// #[derive(Serialize, Clone)]
// struct RegisteredUser {
//     id: UserId,
//     name: String,
// }

// impl Usera for RegisteredUser {}

// fn validate(user: UnvalidatedUser) -> RegisteredUser {
//     RegisteredUser {
//         id: UserId { id: String::new() },
//         name: user.name,
//     }
// }

// fn mutate<T1: Usera, T2: Usera>(user: T1) -> T2 {
//     if let UnvalidatedUser { name: u } = user {
//         validate(u)
//     } else {
//         panic!("dsd")
//     }
// }

// struct UnvalidatedNote;
// struct UncompletedNote;
// struct CompletedNote;
// struct ArchivedNote;

// fn validate(_: UnvalidatedNote) -> UncompletedNote {}
// fn complete(_: UncompletedNote) -> CompletedNote {}
// fn archive(_: UncompletedNote) -> ArchivedNote {}

// struct NoteStateEvent {
//     state: NoteState,
//     event: NoteEvent,
// }

// enum NoteState {
//     Unvalidated(UnvalidatedNote),
//     UnCompleted(UncompletedNote),
//     Completed(CompletedNote),
//     Archived(ArchivedNote),
// }
// impl NoteState {
//     // TODO: From関数
//     fn mutate(self, cmd: Command) -> Result<NoteStateEvent, ()> {
//         match (self, cmd) {
//             (NoteState::Unvalidated(note), Command::Validate) => validate(note)
//                 .map(|note| NoteState::UnCompleted(note))
//                 .and_then(|state| {
//                     Ok(NoteStateEvent {
//                         state: state,
//                         event: NoteEvent::Validated,
//                     })
//                 }),
//             _ => Err(()),
//         }
//     }
// }

// #[derive(PartialEq)]
// enum NoteEvent {
//     Validated,
//     Completed,
//     Archived,
// }

// struct UnvalidatedNote {}
// struct UncompletedNote {}
// struct CompletedNote {}
// struct ArchivedNote {}

// // TODO: Domain, DomainEventを返す
// fn validate(note: UnvalidatedNote) -> Result<UncompletedNote, ()> {
//     Ok(UncompletedNote {})
// }
// fn complete(note: UncompletedNote) -> CompletedNote {
//     CompletedNote {}
// }
// fn archive(note: CompletedNote) -> ArchivedNote {
//     ArchivedNote {}
// }

// fn apply_event(state: NoteState, event: &NoteEvent) -> Result<NoteState, ()> {
//     match (state, event) {
//         (NoteState::Unvalidated(note), NoteEvent::Validated) => {
//             validate(note).and_then(|note| Ok(NoteState::UnCompleted(note)))
//         }
//         (_, _) => Err(()),
//     }
// }

// fn store_event(state: NoteStateEvent) -> Result<NoteState, ()> {
//     Ok(state.state)
// }

// enum Command {
//     Validate,
//     Complete,
//     Archive,
// }

// struct Input {
//     command: Command,
//     args: Vec<String>,
// }

// #[derive(Debug)]
// struct Payload {
//     message: String,
// }

// fn get_events() -> Result<Vec<NoteEvent>, ()> {
//     Ok(vec![NoteEvent::Validated])
// }

// fn handle(input: Input) -> Result<Payload, ()> {
//     let events = get_events().expect("");
//     let default = NoteState::Unvalidated(UnvalidatedNote {});
//     let state: NoteState = events.into_iter().fold(default, |state, event| {
//         apply_event(state, &event).expect("msg")
//     });

//     match input.command {
//         Command::Validate => state
//             .mutate(input.command)
//             .or_else(|_| {
//                 println!("command error");
//                 return Err(());
//             })
//             .and_then(store_event)
//             .map(|_| Payload {
//                 message: String::from("success"),
//             }),
//         _ => Ok(Payload {
//             message: String::from("invalid command"),
//         }),
//     }
// }

// fn main() {
//     let cmd = Input {
//         command: Command::Validate,
//         args: vec![],
//     };
//     let result = handle(cmd);
//     println!("{:?}", result)
// }
