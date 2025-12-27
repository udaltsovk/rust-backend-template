use proc_macro::TokenStream;

mod domain_type;
mod instrument_all;

use crate::{domain_type::domain_type2, instrument_all::instrument_all2};

#[proc_macro_derive(DomainType)]
pub fn domain_type(input: TokenStream) -> TokenStream {
    domain_type2(input.into()).into()
}

#[proc_macro_attribute]
pub fn instrument_all(attr: TokenStream, item: TokenStream) -> TokenStream {
    instrument_all2(attr.into(), item.into()).into()
}
