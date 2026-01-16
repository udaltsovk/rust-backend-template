use domain::session::{Session, entity::SessionEntity};
use lib::{chrono::Utc, uuid::Uuid};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum JWTRole {
    User,
}

impl JWTRole {
    const fn from_session_entity(entity: &SessionEntity) -> (Self, Uuid) {
        use SessionEntity as E;
        match entity {
            E::User(id) => (Self::User, id.value),
        }
    }

    fn into_session_entity(self, id: Uuid) -> SessionEntity {
        use SessionEntity as E;
        match self {
            Self::User => E::User(id.into()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
    sub: Uuid,
    role: JWTRole,
    jti: Uuid,
}

impl From<Session> for Claims {
    fn from(session: Session) -> Self {
        let current_time =
            usize::try_from(Utc::now().timestamp()).unwrap_or(usize::MAX);
        let (role, sub) = JWTRole::from_session_entity(&session.entity);
        Self {
            exp: current_time.saturating_add(Session::LIFETIME),
            iat: current_time,
            sub,
            role,
            jti: session.id.value,
        }
    }
}

impl From<Claims> for Session {
    fn from(cl: Claims) -> Self {
        Self {
            id: cl.jti.into(),
            entity: cl.role.into_session_entity(cl.sub),
        }
    }
}
