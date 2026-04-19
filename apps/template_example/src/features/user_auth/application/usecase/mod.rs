pub mod session;

pub trait AuthUseCases =
    session::CreateSessionUsecase
        + session::GetSessionFromTokenUsecase;
