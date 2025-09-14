use domain::client::gender::ClientGender;
use sqlx::Type;

#[derive(Type, Debug)]
#[sqlx(type_name = "client_gender", rename_all = "lowercase")]
pub enum StoredClientGender {
    Male,
    Female,
}
impl From<StoredClientGender> for ClientGender {
    fn from(g: StoredClientGender) -> Self {
        use StoredClientGender as G;
        match g {
            G::Male => Self::Male,
            G::Female => Self::Female,
        }
    }
}
impl From<ClientGender> for StoredClientGender {
    fn from(g: ClientGender) -> Self {
        use ClientGender as G;
        match g {
            G::Male => Self::Male,
            G::Female => Self::Female,
        }
    }
}
