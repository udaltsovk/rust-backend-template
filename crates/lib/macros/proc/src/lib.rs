use proc_macro::TokenStream;

mod domain_type;

use domain_type::domain_type;

#[proc_macro_derive(DomainType)]
pub fn derive_domain_type(input: TokenStream) -> TokenStream {
    domain_type(input)
}
