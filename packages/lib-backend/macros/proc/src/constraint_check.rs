use proc_macro2::TokenStream;
use quote::quote;
use syn::{ImplItem, ItemImpl, Type, parse2};

pub fn constraint_check2(
    attr: TokenStream,
    item: TokenStream,
) -> syn::Result<TokenStream> {
    let target: Type = parse2(attr).map_err(|_| {
        syn::Error::new_spanned(
            &item,
            "`constraint_check` requires the constrained type, e.g. \
             `#[constraint_check(String)]`",
        )
    })?;

    let item: ItemImpl = parse2(item)?;

    let is_valid = item
        .items
        .iter()
        .find_map(|entry| match entry {
            ImplItem::Fn(method) if method.sig.ident == "is_valid" => {
                Some(method)
            },
            _ => None,
        })
        .ok_or_else(|| {
            syn::Error::new_spanned(
                &item,
                "`constraint_check` requires an `is_valid` method",
            )
        })?;

    let call = if is_valid.sig.receiver().is_some() {
        quote!(self.is_valid(value))
    } else {
        quote!(Self::is_valid(value))
    };

    let self_ty = item.self_ty.as_ref();
    let (impl_generics, _, where_clause) = item.generics.split_for_impl();

    Ok(quote! {
        #item

        impl #impl_generics Constraint<#target> for #self_ty
            #where_clause
        {
            fn check(&self, value: &#target) -> Validation {
                self.validate(#call, value)
            }
        }
    })
}
