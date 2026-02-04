use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    Attribute, ImplItem, ItemImpl, LitStr, Type, meta::parser,
    parse::Parser as _, parse_quote, parse2,
};

pub fn instrument_all2(
    attrs: TokenStream2,
    stream: TokenStream2,
) -> TokenStream2 {
    let mut prefix: Option<LitStr> = None;
    let mut level: Option<LitStr> = None;

    let arg_parser = parser(|meta| match meta.value() {
        Ok(value) if meta.path.is_ident("prefix") => {
            prefix = Some(value.parse()?);
            Ok(())
        },
        Ok(value) if meta.path.is_ident("level") => {
            level = Some(value.parse()?);
            Ok(())
        },

        _ => Err(meta.error("unsupported argument")),
    });

    if let Err(e) = arg_parser.parse2(attrs) {
        return e.to_compile_error();
    }

    let mut item_impl: ItemImpl = match parse2(stream) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error(),
    };

    let prefix = prefix.unwrap_or_else(|| {
        let default_prefix = match &*item_impl.self_ty {
            Type::Path(path) => path
                .path
                .segments
                .last()
                .map(|segment| segment.ident.to_string())
                .unwrap_or_default(),
            _ => String::new(),
        };
        LitStr::new(&default_prefix, Span::call_site())
    });

    let level = level.unwrap_or_else(|| LitStr::new("info", Span::call_site()));

    item_impl
        .items
        .iter_mut()
        .filter_map(|item| {
            if let ImplItem::Fn(item_fn) = item {
                Some(item_fn)
            } else {
                None
            }
        })
        .filter(|item_fn| {
            !item_fn.attrs.iter().any(|attr| {
                let path = attr.path();
                path.is_ident("instrument")
                    || path
                        .segments
                        .last()
                        .is_some_and(|segment| segment.ident == "instrument")
            })
        })
        .for_each(|item_fn| {
            let fn_name = item_fn.sig.ident.to_string();
            let name = if prefix.value().is_empty() {
                fn_name
            } else {
                format!("{}::{}", prefix.value(), fn_name)
            };

            let a: Attribute = parse_quote! {
                #[tracing::instrument(name = #name, level = #level, skip_all)]
            };
            item_fn.attrs.push(a);
        });

    quote!(#item_impl)
}
