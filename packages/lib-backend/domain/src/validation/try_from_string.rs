// TODO: make this a derive macro
#[macro_export]
macro_rules! impl_try_from_string {
    (
        enum = $enum: ident,
        field = $field: literal $(,)*
    ) => {
        $crate::impl_try_from_string!(
            enum = $enum,
            field = $field,
            none_msg = $crate::impl_try_from_external_input!(@default_none_msg),
            missing_msg = $crate::impl_try_from_external_input!(@default_missing_msg),
        );
    };
    (
        enum = $enum: ident,
        field = $field: literal,
        none_msg = $none_msg: expr $(,)*
    ) => {
        $crate::impl_try_from_string!(
            enum = $enum,
            field = $field,
            none_msg = $none_msg,
            missing_msg = $crate::impl_try_from_external_input!(@default_missing_msg),
        );
    };
    (
        enum = $enum: ident,
        field = $field: literal,
        missing_msg = $missing_msg: expr $(,)*
    ) => {
        $crate::impl_try_from_string!(
            enum = $enum,
            field = $field,
            none_msg = $crate::impl_try_from_external_input!(@default_none_msg),
            missing_msg = $missing_msg,
        );
    };
    (
        enum = $enum: ident,
        field = $field: literal,
        none_msg = $none_msg: expr,
        missing_msg = $missing_msg: expr $(,)*
    ) => {
        impl $enum {
            fn parse_error() -> &'static str {
                static ERROR: std::sync::OnceLock<String> =
                    std::sync::OnceLock::new();
                ERROR.get_or_init(|| {
                    let variants: Vec<_> = Self::VARIANTS
                        .iter()
                        .map(|variant| format!("`{variant}`"))
                        .collect();

                    let (last, rest) = variants
                        .split_last()
                        .expect("enum should have at least one variant");

                    let parts: String = rest
                        .iter()
                        .cloned()
                        .intersperse(", ".to_string())
                        .collect();

                    let last_part = if rest.is_empty() {
                        last.clone()
                    } else {
                        format!(" or {last}")
                    };

                    format!("must be {parts}{last_part}")
                })
            }
        }

        impl TryFrom<String> for $enum {
            type Error = $crate::validation::error::ValidationErrors;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                value.parse().map_err(|_| {
                    $crate::validation::error::ValidationErrors::with_error(
                        $field,
                        Self::parse_error(),
                        value,
                    )
                })
            }
        }

        $crate::impl_try_from_external_input!(
            domain_type = $enum,
            input_type = String,
            field = $field,
            type_mismatch_fn = |_| Self::parse_error(),
            none_msg = $none_msg,
            missing_msg = $missing_msg,
        );
    };
}
