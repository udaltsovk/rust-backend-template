use application::usecase::{session::SessionUseCase, user::UserUseCase};

pub trait ModulesExt: Clone + Send + Sync + 'static {
    fn user_usecase(&self) -> &impl UserUseCase;

    fn session_usecase(&self) -> &impl SessionUseCase;
}
