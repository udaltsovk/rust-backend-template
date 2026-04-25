use entrait::Impl;
use lib::bootstrap::metadata::{
    DotenvExample, MetadataSaver as _, MetadataSaverResult,
};
use template_example::{
    AppConfig, bootstrappers::api::rest, modules::Modules,
};

fn main() -> MetadataSaverResult {
    let app_name = "template_example";

    DotenvExample::<AppConfig>::default()
        .save_as(app_name)?;

    rest::router::<Impl<Modules>>()
        .into_openapi()
        .save_as(app_name)?;

    Ok(())
}
