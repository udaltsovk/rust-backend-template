use proc_macro::TokenStream;

mod domain_type;
mod instrument;

use domain_type::domain_type;
use instrument::instrument_all as instrument_all_impl;

#[proc_macro_derive(DomainType)]
pub fn derive_domain_type(input: TokenStream) -> TokenStream {
    domain_type(input)
}

#[proc_macro_attribute]
pub fn instrument_all(attr: TokenStream, item: TokenStream) -> TokenStream {
    instrument_all_impl(attr, item)
}
