use lib::bootstrap::metadata::{
    DotenvExample, MetadataSaver as _, MetadataSaverResult,
};
use presentation::api::rest::routes;
use template_example::{AppConfig, Modules};

fn main() -> MetadataSaverResult {
    let app_name = "template_example";

    DotenvExample::<AppConfig>::default().save_as(app_name)?;

    routes::router::<Modules>()
        .into_openapi()
        .save_as(app_name)?;

    Ok(())
}
