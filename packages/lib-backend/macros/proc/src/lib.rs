use std::path::PathBuf;

use proc_macro::TokenStream;

mod domain_type;
mod feature_postgres;
mod instrument_all;

use crate::{
    domain_type::domain_type2,
    feature_postgres::{
        feature_postgres_migrator2,
        feature_postgres_query_file_as2,
    },
    instrument_all::instrument_all2,
};

#[proc_macro_derive(DomainType)]
pub fn domain_type(input: TokenStream) -> TokenStream {
    domain_type2(input.into()).into()
}

#[proc_macro]
pub fn migrate(input: TokenStream) -> TokenStream {
    let span = proc_macro::Span::call_site();
    let invocation_file = span
        .local_file()
        .unwrap_or_else(|| PathBuf::from(span.file()));

    let input = proc_macro2::TokenStream::from(input);

    feature_postgres_migrator2(&invocation_file, &input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn query_file_as(input: TokenStream) -> TokenStream {
    let span = proc_macro::Span::call_site();
    let invocation_file = span
        .local_file()
        .unwrap_or_else(|| PathBuf::from(span.file()));

    feature_postgres_query_file_as2(
        &invocation_file,
        input.into(),
    )
    .unwrap_or_else(syn::Error::into_compile_error)
    .into()
}

#[proc_macro_attribute]
pub fn instrument_all(
    attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    instrument_all2(attr.into(), item.into()).into()
}
