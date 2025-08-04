use crate::domain::{
    Id,
    client::{
        age::ClientAge, gender::ClientGender, location::ClientLocation,
        login::ClientLogin,
    },
};

pub mod age;
pub mod gender;
pub mod location;
pub mod login;

pub struct Client {
    pub id: Id<Client>,
    pub login: ClientLogin,
    pub age: ClientAge,
    pub gender: ClientGender,
    pub location: ClientLocation,
}

pub struct UpsertClient {
    pub id: Id<Client>,
    pub login: ClientLogin,
    pub age: ClientAge,
    pub gender: ClientGender,
    pub location: ClientLocation,
}
