use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse2};

pub fn domain_type2(input: TokenStream2) -> TokenStream2 {
    let input: DeriveInput = match parse2(input) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error(),
    };

    let ident = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(s) => &s.fields,
        _ => panic!("DomainType can only be derived for structs"),
    };

    let (from_impl, value_impl, value_mut_impl, inner_type) = match fields {
        Fields::Named(_) => {
            panic!("DomainType can only be derived for unnamed field structs")
        },
        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            let inner_type = &fields
                .unnamed
                .first()
                .expect("We've checked for field count so it's safe")
                .ty;
            (
                quote! { domain_type.0 },
                quote! { &self.0 },
                quote! { &mut self.0 },
                inner_type,
            )
        },
        _ => panic!("DomainType requires exactly one field in the struct"),
    };

    quote! {
        impl From<#ident #ty_generics> for #inner_type #where_clause {
            fn from(domain_type: #ident #ty_generics) -> Self {
                #from_impl
            }
        }

        impl AsRef<#inner_type> for #ident #ty_generics #where_clause {
            fn as_ref(&self) -> &#inner_type {
                #value_impl
            }
        }


        impl AsMut<#inner_type> for #ident #ty_generics #where_clause {
            fn as_mut(&mut self) -> &mut #inner_type {
                #value_mut_impl
            }
        }

        impl #impl_generics lib::domain::DomainType<#inner_type> for #ident #ty_generics #where_clause {
        }
    }
}
