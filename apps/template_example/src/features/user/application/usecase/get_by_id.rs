use entrait::entrait;
use lib::domain::Id;
use tracing::instrument;

use super::{UserUseCaseError, UserUseCaseResult};
use crate::features::user::{
    application::repository::UserRepository, domain::User,
};

#[entrait(pub GetUserByIdUsecase)]
#[instrument(skip(deps))]
async fn get_user_by_id<Deps>(
    deps: &Deps,
    id: Id<User>,
) -> UserUseCaseResult<User>
where
    Deps: UserRepository,
{
    UserRepository::find_user_by_id(deps, id)
        .await?
        .ok_or(UserUseCaseError::NotFoundById(id))
}
