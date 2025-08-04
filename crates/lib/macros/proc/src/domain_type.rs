use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

pub fn derive_domain_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(s) => &s.fields,
        _ => panic!("DomainType can only be derived for structs"),
    };

    let (value_impl, value_mut_impl, into_inner_impl) = match fields {
        Fields::Named(_) => {
            panic!("DomainType can only be derived for unnamed field structs")
        },
        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => (
            quote! { &self.0 },
            quote! { &mut self.0 },
            quote! { self.0 },
        ),
        _ => panic!("DomainType requires exactly one field in the struct"),
    };

    let inner_type = match fields {
        Fields::Unnamed(fields) => {
            &fields
                .unnamed
                .first()
                .expect("We've checked for field count so it's safe")
                .ty
        },
        _ => unreachable!(),
    };

    let expanded = quote! {
        impl #impl_generics crate::domain::DomainType<#inner_type> for #ident #ty_generics #where_clause {
            fn value(&self) -> &#inner_type {
                #value_impl
            }

            fn value_mut(&mut self) -> &mut #inner_type {
                #value_mut_impl
            }

            fn into_inner(self) -> #inner_type {
                #into_inner_impl
            }
        }
    };

    TokenStream::from(expanded)
}
