use std::sync::LazyLock;

use lib::infrastructure::persistence::{
    redis::Namespace, repository_impl_struct,
};

mod session;

static META_NAMESPACE: LazyLock<Namespace> =
    LazyLock::new(|| Namespace::new("template_example").nest("monolyth"));

repository_impl_struct!(Redis);
