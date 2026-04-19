use entrait::entrait;
use lib::{domain::Id, tap::Pipe as _};
use tracing::instrument;

use super::UserUseCaseResult;
use crate::features::user::{
    application::repository::UserRepository, domain::User,
};

#[entrait(pub FindUserByIdUsecase)]
#[instrument(skip(deps))]
async fn find_user_by_id<Deps>(
    deps: &Deps,
    id: Id<User>,
) -> UserUseCaseResult<Option<User>>
where
    Deps: UserRepository,
{
    UserRepository::find_user_by_id(deps, id)
        .await?
        .pipe(Ok)
}
