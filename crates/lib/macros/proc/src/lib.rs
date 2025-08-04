use proc_macro::TokenStream;

mod domain_type;
use domain_type::derive_domain_type as derive_domain_type_fn;

#[proc_macro_derive(DomainType)]
pub fn derive_domain_type(input: TokenStream) -> TokenStream {
    derive_domain_type_fn(input)
}
