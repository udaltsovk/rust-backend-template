use domain::user::User;
use entrait::entrait;
use lib::domain::Id;
use tracing::instrument;

use crate::{
    repository::user::UserRepository,
    usecase::user::error::{UserUseCaseError, UserUseCaseResult},
};

#[entrait(pub GetUserByIdUsecase)]
#[instrument(skip(app))]
async fn get_user_by_id<App>(app: &App, id: Id<User>) -> UserUseCaseResult<User>
where
    App: UserRepository,
{
    app.find_user_by_id(id)
        .await?
        .ok_or(UserUseCaseError::NotFoundById(id))
}
