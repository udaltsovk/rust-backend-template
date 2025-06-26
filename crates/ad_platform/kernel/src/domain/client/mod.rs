use crate::domain::{Id, client::gender::ClientGender};

pub mod gender;

pub struct Client {
    pub id: Id<Client>,
    pub login: String,
    pub age: u16,
    pub gender: ClientGender,
    pub location: String,
}

pub struct UpsertClient {
    pub id: Id<Client>,
    pub login: String,
    pub age: u16,
    pub gender: ClientGender,
    pub location: String,
}
