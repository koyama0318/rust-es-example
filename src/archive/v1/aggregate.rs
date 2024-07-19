// use serde::Serialize;

// struct DomainEvent {
//     aggregate_id: String,
//     event_type: String,
// }

// enum UserEvent {
//     UserRegistered,
//     UserCancelled,
// }

// impl UserEvent {
//     fn from(domain_event: DomainEvent) -> Self {
//         match domain_event.event_type.as_str() {
//             "UserRegistered" => UserEvent::UserRegistered,
//             "UserCancelled" => UserEvent::UserCancelled,
//             _ => panic!("Unknown event type"),
//         }
//     }

//     fn make_event(&self, aggregate_id: String) -> DomainEvent {
//         match self {
//             UserEvent::UserRegistered => DomainEvent {
//                 aggregate_id,
//                 event_type: String::from("UserRegistered"),
//             },
//             UserEvent::UserCancelled => DomainEvent {
//                 aggregate_id,
//                 event_type: String::from("UserCancelled"),
//             },
//         }
//     }
// }

// trait Aggregate {
//     fn aggregate_id(&self) -> String;
//     fn snapshot(&self) -> String;
//     fn replay(events: Vec<DomainEvent>) -> Self;
// }

// struct UserAggregate<T: User> {
//     user: T,
// }

// impl<T: User> Aggregate for UserAggregate<T> {
//     fn aggregate_id(&self) -> String {
//         format!("USER#{}", self.user.id().id)
//     }

//     fn snapshot(&self) -> String {
//         serde_json::to_string(&self.user).unwrap()
//     }

//     fn replay(events: Vec<DomainEvent>) -> Self {
//         let mut user = UnvalidatedUser {
//             name: String::new(),
//         };

//         for event in events {
//             match UserEvent::from(event) {
//                 UserEvent::UserRegistered => {
//                     user = RegisteredUser {
//                         id: UserId {
//                             id: "id".to_string(),
//                         },
//                         name: "name".to_string(),
//                     };
//                 }
//             }
//         }

//         let registered_user = if let RegisteredUser { id, name } = user {
//             RegisteredUser { id, name }
//         } else {
//             panic!("Unexpected user type")
//         };

//         UserAggregate::new(registered_user)
//     }
// }

// impl<T: User> UserAggregate<T> {
//     fn id(&self) -> UserId {
//         self.user.id()
//     }
// }

// trait User: Serialize {
//     fn id(&self) -> UserId;
// }

// #[derive(Serialize, Clone)]
// struct UserId {
//     id: String,
// }

// #[derive(Serialize)]
// struct UnvalidatedUser {
//     name: String,
// }

// impl User for UnvalidatedUser {
//     fn id(&self) -> UserId {
//         UserId {
//             id: String::from("unvalidated_user_id"),
//         }
//     }
// }

// #[derive(Serialize)]
// struct RegisteredUser {
//     id: UserId,
//     name: String,
// }

// impl User for RegisteredUser {
//     fn id(&self) -> UserId {
//         self.id.clone()
//     }
// }

// #[derive(Serialize)]
// struct CancelledUser {
//     id: UserId,
//     name: String,
// }

// impl User for CancelledUser {
//     fn id(&self) -> UserId {
//         self.id.clone()
//     }
// }

// // use serde_json::to_string;

// // pub trait Aggregate: Sized {
// //     fn replay(events: Vec<DomainEvent>) -> Self
// //     where
// //         Self: Default,
// //     {
// //         let mut aggregate = Self::default();
// //         for event in events {
// //             aggregate = aggregate.process(event);
// //         }
// //         aggregate
// //     }

// //     fn process(&mut self, event: DomainEvent) -> Self;

// //     fn get_aggregate_id(&self) -> String;
// //     fn make_snapshot(&self) -> String;
// // }

// // pub trait NoteAggregate: Aggregate + Serialize {
// //     fn get_aggregate_id(&self) -> String {
// //         format!("NOTE#{}", self.get_id())
// //     }

// //     fn make_snapshot(&self) -> String {
// //         to_string(self).unwrap_or_else(|_| String::from("{}"))
// //     }

// //     fn get_id(&self) -> String;
// // }

// // #[derive(Default, Serialize)]
// // pub struct UnvalidatedNote {
// //     // Note struct fields here
// // }

// // impl NoteAggregate for UnvalidatedNote {
// //     fn get_id(&self) -> String {
// //         // Return the id of the note
// //         String::from("note_id") // Placeholder implementation
// //     }
// // }

// // impl Aggregate for UnvalidatedNote {
// //     fn get_aggregate_id(&self) -> String {
// //         <Self as NoteAggregate>::get_aggregate_id(self)
// //     }

// //     fn process(&mut self, event: DomainEvent) -> Self {
// //         // Process the event and return the new state
// //         // This is a placeholder implementation
// //         UnvalidatedNote {}
// //     }

// //     fn make_snapshot(&self) -> String {
// //         to_string(self).unwrap_or_else(|_| String::from("{}"))
// //     }
// // }

// // pub struct DomainEvent {
// //     id: String,
// //     aggregate_id: String,
// //     event_type: String,
// //     event_data: String,
// //     timestamp: u64,
// //     version: u64,
// // }
