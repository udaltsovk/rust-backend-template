use domain::user::User;
use entrait::entrait;
use lib::{domain::Id, tap::Pipe as _};
use tracing::instrument;

use crate::{
    repository::user::UserRepository,
    usecase::user::error::{UserUseCaseError, UserUseCaseResult},
};

#[entrait(pub FindUserByIdUsecase)]
#[instrument(skip(app))]
async fn find_user_by_id<App>(
    app: &App,
    id: Id<User>,
) -> UserUseCaseResult<Option<User>>
where
    App: UserRepository,
{
    app.find_user_by_id(id)
        .await
        .map_err(UserUseCaseError::Infrastructure)?
        .pipe(Ok)
}
