use proc_macro2::TokenStream;
use quote::quote;
use syn::{Fields, ItemStruct, Type, parse_quote, parse2};

pub fn constraint2(item: TokenStream) -> syn::Result<TokenStream> {
    let mut item: ItemStruct = parse2(item)?;

    let Fields::Named(fields) = &mut item.fields else {
        return Err(syn::Error::new_spanned(
            &item,
            "`constraint` requires a struct with named fields",
        ));
    };

    let is_err_fn = |field: &syn::Field| {
        field.ident.as_ref().is_some_and(|ident| ident == "err_fn")
    };

    let Some(err_fn) = fields.named.iter().find(|f| is_err_fn(f)).cloned()
    else {
        return Err(syn::Error::new_spanned(
            &item,
            "`constraint` requires an `err_fn` field",
        ));
    };

    let Type::FnPtr(signature) = &err_fn.ty else {
        return Err(syn::Error::new_spanned(
            &err_fn.ty,
            "`err_fn` must be a function pointer",
        ));
    };

    let Some(value_ty) = signature.inputs.first().map(|arg| &arg.ty) else {
        return Err(syn::Error::new_spanned(
            &err_fn.ty,
            "`err_fn` must take the rejected value as its first argument",
        ));
    };

    let extra = signature.inputs.iter().skip(1);
    let state = fields.named.iter().filter(|f| !is_err_fn(f));

    let args: Vec<TokenStream> = extra
        .zip(state)
        .map(|(arg, field)| {
            let name = field.ident.clone();

            if matches!(arg.ty, Type::Reference(_)) {
                quote!(&self.#name)
            } else {
                quote!(self.#name)
            }
        })
        .collect();

    let value_ty = value_ty.clone();

    for field in &mut fields.named {
        if is_err_fn(field) {
            field.attrs.push(parse_quote!(#[builder(start_fn)]));
        }
    }

    item.attrs.push(parse_quote!(#[derive(::bon::Builder)]));
    item.attrs
        .push(parse_quote!(#[builder(derive(Clone), start_fn = with_err)]));

    let name = &item.ident;
    let (impl_generics, ty_generics, where_clause) =
        item.generics.split_for_impl();

    Ok(quote! {
        #item

        impl #impl_generics #name #ty_generics #where_clause {
            fn validate(
                &self,
                valid: bool,
                value: #value_ty,
            ) -> Validation {
                if valid {
                    Validation::Valid
                } else {
                    Validation::Invalid((self.err_fn)(value #(, #args)*))
                }
            }
        }
    })
}
