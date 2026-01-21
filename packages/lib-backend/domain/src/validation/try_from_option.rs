#[macro_export]
macro_rules! try_from_option {
    (
      domain_type = $domain_type: ident,
      from_ty = $from_ty: ident,
      constraints = $constraints: ident $(,)*
    ) => {
        impl TryFrom<Option<$from_ty>> for $domain_type {
            type Error = $crate::validation::error::ValidationErrors;

            fn try_from(
                value: Option<$from_ty>,
            ) -> Result<Self, $crate::validation::error::ValidationErrors> {
                value
                    .map(Self::try_from)
                    .transpose()?
                    .ok_or_else(|| $constraints.none_error())
            }
        }
    };
}
