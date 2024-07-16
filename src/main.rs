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
