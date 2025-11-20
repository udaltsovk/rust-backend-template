use chrono::Utc;
use domain::session::{Session, entity::SessionEntity};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const TOKEN_LIFETIME: usize = 60 * 60 * 24 * 3_usize; // 3 days

#[derive(Serialize, Deserialize)]
pub enum JWTAud {
    Client,
}

impl JWTAud {
    const fn from_session_entity(entity: &SessionEntity) -> (Self, Uuid) {
        use SessionEntity as E;
        match entity {
            E::Client(id) => (Self::Client, id.value),
        }
    }

    fn into_session_entity(self, id: Uuid) -> SessionEntity {
        use SessionEntity as E;
        match self {
            Self::Client => E::Client(id.into()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
    sub: Uuid,
    aud: JWTAud,
    jti: Uuid,
}

impl From<Session> for Claims {
    fn from(session: Session) -> Self {
        let current_time =
            usize::try_from(Utc::now().timestamp()).unwrap_or(usize::MAX);
        let (aud, sub) = JWTAud::from_session_entity(&session.entity);
        Self {
            exp: current_time.saturating_add(TOKEN_LIFETIME),
            iat: current_time,
            sub,
            aud,
            jti: session.id.value,
        }
    }
}

impl From<Claims> for Session {
    fn from(cl: Claims) -> Self {
        Self {
            id: cl.jti.into(),
            entity: cl.aud.into_session_entity(cl.sub),
        }
    }
}
