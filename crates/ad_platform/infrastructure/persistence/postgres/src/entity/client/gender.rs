use kernel::domain::client::gender::ClientGender;
use sqlx::Type;

#[derive(Type, Debug)]
#[sqlx(type_name = "client_gender", rename_all = "lowercase")]
pub enum StoredClientGender {
    MALE,
    FEMALE,
}
impl From<StoredClientGender> for ClientGender {
    fn from(g: StoredClientGender) -> Self {
        use StoredClientGender as G;
        match g {
            G::MALE => Self::Male,
            G::FEMALE => Self::Female,
        }
    }
}
impl From<ClientGender> for StoredClientGender {
    fn from(g: ClientGender) -> Self {
        use ClientGender as G;
        match g {
            G::Male => Self::MALE,
            G::Female => Self::FEMALE,
        }
    }
}
