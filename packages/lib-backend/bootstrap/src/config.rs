use fromenv::__private::{FromEnv, FromEnvBuilder};

pub trait ConfigExt: FromEnv {
    type Target;

    fn load() -> Self::Target;
}

impl<C> ConfigExt for C
where
    C: FromEnv,
{
    type Target = <C::FromEnvBuilder as FromEnvBuilder>::Target;

    fn load() -> Self::Target {
        #[cfg(debug_assertions)]
        match dotenvy::dotenv() {
            Ok(_) => {
                tracing::debug!("Successfully loaded .env");
            },
            Err(_) => {
                tracing::debug!("Failed to load .env");
            },
        };

        Self::from_env()
            .finalize()
            .unwrap_or_else(|err| panic!("{err}"))
    }
}
