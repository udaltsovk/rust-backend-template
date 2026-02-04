use std::{
    any::type_name,
    collections::HashMap,
    sync::{LazyLock, RwLock},
};

static TYPE_NAMES: LazyLock<RwLock<HashMap<&'static str, &'static str>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub fn get_type_name<T>() -> &'static str {
    let type_name = type_name::<T>();
    TYPE_NAMES
        .write()
        .expect("RwLock should not be poisoned")
        .entry(type_name)
        .or_insert_with(|| {
            type_name
                .split_once('<')
                .map_or(type_name, |(path, _)| path)
                .split("::")
                .last()
                .unwrap_or(type_name)
        })
}

// TODO: make this a derive macro
#[macro_export]
macro_rules! impl_try_from_external_input {
    (@default_type_mismatch_fn) => {|expected| format!("must be {expected}")};

    (@default_none_msg) => {"must not be null"};

    (@default_missing_msg) => {"must be present"};

    (
        domain_type = $domain_type: ident,
        input_type = $input_type: path $(,)?
    ) => {
        $crate::impl_try_from_external_input!(
            domain_type = $domain_type,
            input_type = $input_type,
            type_mismatch_fn = impl_try_from_external_input!(@default_type_mismatch_fn),
            none_msg = impl_try_from_external_input!(@default_none_msg),
            missing_msg = impl_try_from_external_input!(@default_missing_msg),
        );
    };
    (
        domain_type = $domain_type: ident,
        input_type = $input_type: path $(,)?
    ) => {
        $crate::impl_try_from_external_input!(
            domain_type = $domain_type,
            input_type = $input_type,
            type_mismatch_fn = impl_try_from_external_input!(@default_type_mismatch_fn),
            none_msg = impl_try_from_external_input!(@default_none_msg),
            missing_msg = impl_try_from_external_input!(@default_missing_msg),
        );
    };
    (
        domain_type = $domain_type: ident,
        input_type = $input_type: path,
        type_mismatch_fn = $type_mismatch_fn: expr $(,)?
    ) => {
        $crate::impl_try_from_external_input!(
            domain_type = $domain_type,
            input_type = $input_type,
            type_mismatch_fn = $type_mismatch_fn,
            none_msg = impl_try_from_external_input!(@default_none_msg),
            missing_msg = impl_try_from_external_input!(@default_missing_msg),
        );
    };
    (
        domain_type = $domain_type: ident,
        input_type = $input_type: path,
        none_msg = $none_msg: expr $(,)?
    ) => {
        $crate::impl_try_from_external_input!(
            domain_type = $domain_type,
            input_type = $input_type,
            type_mismatch_fn = impl_try_from_external_input!(@default_type_mismatch_fn),
            none_msg = $none_msg,
            missing_msg = impl_try_from_external_input!(@default_missing_msg),
        );
    };
    (
        domain_type = $domain_type: ident,
        input_type = $input_type: path,
        missing_msg = $missing_msg: expr $(,)?
    ) => {
        $crate::impl_try_from_external_input!(
            domain_type = $domain_type,
            input_type = $input_type,
            type_mismatch_fn = impl_try_from_external_input!(@default_type_mismatch_fn),
            none_msg = impl_try_from_external_input!(@default_none_msg),
            missing_msg = $missing_msg,
        );
    };
    (
        domain_type = $domain_type: ident,
        input_type = $input_type: path,
        none_msg = $none_msg: expr,
        missing_msg = $missing_msg: expr $(,)?
    ) => {
        $crate::impl_try_from_external_input!(
            domain_type = $domain_type,
            input_type = $input_type,
            type_mismatch_fn = impl_try_from_external_input!(@default_type_mismatch_fn),
            none_msg = $none_msg,
            missing_msg = $missing_msg,
        );
    };
    (
        domain_type = $domain_type: ident,
        input_type = $input_type: path,
        type_mismatch_fn = $type_mismatch_fn: expr,
        missing_msg = $missing_msg: expr $(,)?
    ) => {
        $crate::impl_try_from_external_input!(
            domain_type = $domain_type,
            input_type = $input_type,
            type_mismatch_fn = $type_mismatch_fn,
            none_msg = impl_try_from_external_input!(@default_none_msg),
            missing_msg = $missing_msg,
        );
    };
    (
        domain_type = $domain_type: ident,
        input_type = $input_type: path,
        type_mismatch_fn = $type_mismatch_fn: expr,
        none_msg = $none_msg: expr $(,)?
    ) => {
        $crate::impl_try_from_external_input!(
            domain_type = $domain_type,
            input_type = $input_type,
            type_mismatch_fn = $type_mismatch_fn,
            none_msg = $none_msg,
            missing_msg = impl_try_from_external_input!(@default_missing_msg),
        );
    };
    (
        domain_type = $domain_type: ident,
        input_type = $input_type: path,
        type_mismatch_fn = $type_mismatch_fn: expr,
        none_msg = $none_msg: expr,
        missing_msg = $missing_msg: expr $(,)?
    ) => {
        impl TryFrom<$crate::validation::ExternalInput<$input_type>>
            for $domain_type
        {
            type Error = $crate::validation::error::ValidationErrors;

            fn try_from(
                input: $crate::validation::ExternalInput<$input_type>,
            ) -> $crate::validation::error::ValidationResult<Self> {
                input.map_or_else(
                    |value| Self::try_from(value).map_err(Into::into),
                    |value| {
                        let expected_type = $crate::validation::get_type_name::<$input_type>();
                        Err($crate::validation::error::ValidationErrors::with_error(
                            ($type_mismatch_fn)(expected_type),
                            value,
                        ))
                    },
                    || Err($crate::validation::error::ValidationErrors::with_error(
                        $none_msg,
                        None::<()>
                    )),
                    || Err($crate::validation::error::ValidationErrors::with_error(
                        $missing_msg,
                        None::<()>
                    )),
                )
            }
        }
    };
}
