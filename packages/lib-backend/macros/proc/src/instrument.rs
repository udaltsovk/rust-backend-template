use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemImpl, LitStr, parse_macro_input};

pub fn instrument_all(attr: TokenStream, stream: TokenStream) -> TokenStream {
    let repo_name = parse_macro_input!(attr as LitStr);
    let mut impl_block = parse_macro_input!(stream as ItemImpl);

    for item in &mut impl_block.items {
        if let syn::ImplItem::Fn(method) = item {
            let fn_name = method.sig.ident.to_string();
            let full_name = format!("{}::{}", repo_name.value(), fn_name);

            let instrument_attr: syn::Attribute = syn::parse_quote! {
                #[tracing::instrument(name = #full_name, skip_all)]
            };
            method.attrs.push(instrument_attr);
        }
    }

    quote!(#impl_block).into()
}
