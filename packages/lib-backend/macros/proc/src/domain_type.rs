use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

pub fn domain_type(input: TokenStream) -> TokenStream {
    domain_type_internal(input.into()).into()
}

fn domain_type_internal(input: TokenStream2) -> TokenStream2 {
    let input = match syn::parse2::<DeriveInput>(input) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error(),
    };
    domain_type_impl(input)
}

pub fn domain_type_impl(input: DeriveInput) -> TokenStream2 {
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

#[cfg(test)]
mod tests {
    use quote::quote;
    use rstest::rstest;
    use syn::{DeriveInput, parse_quote};

    use super::{domain_type_impl, domain_type_internal};

    #[rstest]
    fn domain_type_basic_string() {
        let input: DeriveInput = parse_quote! {
            struct UserId(String);
        };

        let result_tokens = domain_type_impl(input);
        let result_str = result_tokens.to_string();

        assert!(result_str.contains("impl From < UserId > for String"));
        assert!(result_str.contains("impl AsRef < String > for UserId"));
        assert!(result_str.contains("impl AsMut < String > for UserId"));
        assert!(result_str.contains(
            "impl lib :: domain :: DomainType < String > for UserId"
        ));
    }

    #[rstest]
    fn domain_type_with_uuid() {
        let input: DeriveInput = parse_quote! {
            struct EntityId(uuid::Uuid);
        };

        let result_tokens = domain_type_impl(input);
        let result_str = result_tokens.to_string();

        assert!(result_str.contains("impl From < EntityId > for uuid :: Uuid"));
        assert!(
            result_str.contains("impl AsRef < uuid :: Uuid > for EntityId")
        );
        assert!(
            result_str.contains("impl AsMut < uuid :: Uuid > for EntityId")
        );
        assert!(result_str.contains(
            "impl lib :: domain :: DomainType < uuid :: Uuid > for EntityId"
        ));
    }

    #[rstest]
    fn domain_type_with_generics() {
        let input: DeriveInput = parse_quote! {
            struct Id<T>(uuid::Uuid) where T: Clone;
        };

        let result_tokens = domain_type_impl(input);
        let result_str = result_tokens.to_string();

        assert!(result_str.contains("From < Id < T > > for uuid :: Uuid"));
        assert!(
            result_str.contains(
                "AsRef < uuid :: Uuid > for Id < T > where T : Clone"
            )
        );
        assert!(
            result_str.contains(
                "AsMut < uuid :: Uuid > for Id < T > where T : Clone"
            )
        );
        assert!(result_str.contains("lib :: domain :: DomainType < uuid :: Uuid > for Id < T > where T : Clone"));
    }

    #[rstest]
    fn domain_type_with_complex_generics() {
        let input: DeriveInput = parse_quote! {
            struct Wrapper<A, B>(Vec<(A, B)>) where A: Clone, B: Send + Sync;
        };

        let result_tokens = domain_type_impl(input);
        let result_str = result_tokens.to_string();

        assert!(result_str.contains("From < Wrapper < A , B > > for Vec < (A , B) > where A : Clone , B : Send + Sync"));
        assert!(result_str.contains("AsRef < Vec < (A , B) > > for Wrapper < A , B > where A : Clone , B : Send + Sync"));
        assert!(result_str.contains("AsMut < Vec < (A , B) > > for Wrapper < A , B > where A : Clone , B : Send + Sync"));
    }

    #[rstest]
    fn domain_type_primitive_types() {
        let input: DeriveInput = parse_quote! {
            struct Counter(i32);
        };

        let result_tokens = domain_type_impl(input);
        let result_str = result_tokens.to_string();

        assert!(result_str.contains("impl From < Counter > for i32"));
        assert!(result_str.contains("impl AsRef < i32 > for Counter"));
        assert!(result_str.contains("impl AsMut < i32 > for Counter"));
        assert!(
            result_str.contains(
                "impl lib :: domain :: DomainType < i32 > for Counter"
            )
        );
    }

    #[rstest]
    fn domain_type_generates_correct_from_implementation() {
        let input: DeriveInput = parse_quote! {
            struct Email(String);
        };

        let result_tokens = domain_type_impl(input);
        let result_str = result_tokens.to_string();

        // Verify the From impl extracts the inner value correctly
        assert!(result_str.contains("domain_type . 0"));
    }

    #[rstest]
    fn domain_type_generates_correct_asref_implementation() {
        let input: DeriveInput = parse_quote! {
            struct Name(String);
        };

        let result_tokens = domain_type_impl(input);
        let result_str = result_tokens.to_string();

        // Verify the AsRef impl references the inner value correctly
        assert!(result_str.contains("& self . 0"));
    }

    #[rstest]
    fn domain_type_generates_correct_asmut_implementation() {
        let input: DeriveInput = parse_quote! {
            struct Score(u64);
        };

        let result_tokens = domain_type_impl(input);
        let result_str = result_tokens.to_string();

        // Verify the AsMut impl mutably references the inner value correctly
        assert!(result_str.contains("& mut self . 0"));
    }

    #[rstest]
    #[should_panic(expected = "DomainType can only be derived for structs")]
    fn domain_type_enum_panics() {
        let input: DeriveInput = parse_quote! {
            enum Status {
                Active,
                Inactive,
            }
        };

        domain_type_impl(input);
    }

    #[rstest]
    #[should_panic(expected = "DomainType can only be derived for structs")]
    fn domain_type_union_panics() {
        let input: DeriveInput = parse_quote! {
            union Data {
                int_val: i32,
                float_val: f32,
            }
        };

        domain_type_impl(input);
    }

    #[rstest]
    #[should_panic(
        expected = "DomainType can only be derived for unnamed field structs"
    )]
    fn domain_type_named_fields_panics() {
        let input: DeriveInput = parse_quote! {
            struct User {
                name: String,
                email: String,
            }
        };

        domain_type_impl(input);
    }

    #[rstest]
    #[should_panic(
        expected = "DomainType can only be derived for unnamed field structs"
    )]
    fn domain_type_mixed_fields_panics() {
        let input: DeriveInput = parse_quote! {
            struct Mixed {
                named: String,
            }
        };

        domain_type_impl(input);
    }

    #[rstest]
    #[should_panic(
        expected = "DomainType requires exactly one field in the struct"
    )]
    fn domain_type_multiple_fields_panics() {
        let input: DeriveInput = parse_quote! {
            struct Point(i32, i32);
        };

        domain_type_impl(input);
    }

    #[rstest]
    #[should_panic(
        expected = "DomainType requires exactly one field in the struct"
    )]
    fn domain_type_three_fields_panics() {
        let input: DeriveInput = parse_quote! {
            struct Triple(i32, i32, i32);
        };

        domain_type_impl(input);
    }

    #[rstest]
    #[should_panic(
        expected = "DomainType requires exactly one field in the struct"
    )]
    fn domain_type_no_fields_panics() {
        let input: DeriveInput = parse_quote! {
            struct Empty();
        };

        domain_type_impl(input);
    }

    #[rstest]
    #[should_panic(
        expected = "DomainType requires exactly one field in the struct"
    )]
    fn domain_type_unit_struct_panics() {
        let input: DeriveInput = parse_quote! {
            struct Unit;
        };

        domain_type_impl(input);
    }

    #[rstest]
    fn domain_type_with_visibility_modifiers() {
        let input: DeriveInput = parse_quote! {
            pub struct PublicId(String);
        };

        let result_tokens = domain_type_impl(input);
        let result_str = result_tokens.to_string();

        // Should still generate the correct impls regardless of visibility
        assert!(result_str.contains("impl From < PublicId > for String"));
        assert!(result_str.contains("impl AsRef < String > for PublicId"));
        assert!(result_str.contains("impl AsMut < String > for PublicId"));
        assert!(result_str.contains(
            "impl lib :: domain :: DomainType < String > for PublicId"
        ));
    }

    #[rstest]
    fn domain_type_with_attributes() {
        let input: DeriveInput = parse_quote! {
            #[derive(Debug, Clone)]
            #[allow(dead_code)]
            struct AttributedId(u64);
        };

        let result_tokens = domain_type_impl(input);
        let result_str = result_tokens.to_string();

        // Should generate correct impls regardless of existing attributes
        assert!(result_str.contains("impl From < AttributedId > for u64"));
        assert!(result_str.contains("impl AsRef < u64 > for AttributedId"));
        assert!(result_str.contains("impl AsMut < u64 > for AttributedId"));
        assert!(result_str.contains(
            "impl lib :: domain :: DomainType < u64 > for AttributedId"
        ));
    }

    #[rstest]
    fn domain_type_with_complex_inner_type() {
        let input: DeriveInput = parse_quote! {
            struct ComplexWrapper(std::collections::HashMap<String, Vec<Option<i32>>>);
        };

        let result_tokens = domain_type_impl(input);
        let result_str = result_tokens.to_string();

        assert!(result_str.contains("impl From < ComplexWrapper > for std :: collections :: HashMap < String , Vec < Option < i32 > > >"));
        assert!(result_str.contains("impl AsRef < std :: collections :: HashMap < String , Vec < Option < i32 > > > > for ComplexWrapper"));
        assert!(result_str.contains("impl AsMut < std :: collections :: HashMap < String , Vec < Option < i32 > > > > for ComplexWrapper"));
        assert!(result_str.contains("impl lib :: domain :: DomainType < std :: collections :: HashMap < String , Vec < Option < i32 > > > > for ComplexWrapper"));
    }

    #[rstest]
    fn domain_type_with_lifetime_generics() {
        let input: DeriveInput = parse_quote! {
            struct LifetimeWrapper<'a>(&'a str);
        };

        let result_tokens = domain_type_impl(input);
        let result_str = result_tokens.to_string();

        assert!(
            result_str.contains("From < LifetimeWrapper < 'a > > for & 'a str")
        );
        assert!(
            result_str
                .contains("AsRef < & 'a str > for LifetimeWrapper < 'a >")
        );
        assert!(
            result_str
                .contains("AsMut < & 'a str > for LifetimeWrapper < 'a >")
        );
        assert!(result_str.contains("lib :: domain :: DomainType < & 'a str > for LifetimeWrapper < 'a >"));
    }

    #[rstest]
    fn domain_type_with_const_generics() {
        let input: DeriveInput = parse_quote! {
            struct ArrayWrapper<const N: usize>([u8; N]);
        };

        let result_tokens = domain_type_impl(input);
        let result_str = result_tokens.to_string();

        assert!(
            result_str.contains("From < ArrayWrapper < N > > for [u8 ; N]")
        );
        assert!(
            result_str.contains("AsRef < [u8 ; N] > for ArrayWrapper < N >")
        );
        assert!(
            result_str.contains("AsMut < [u8 ; N] > for ArrayWrapper < N >")
        );
        assert!(result_str.contains(
            "lib :: domain :: DomainType < [u8 ; N] > for ArrayWrapper < N >"
        ));
    }

    #[rstest]
    fn domain_type_internal_handles_invalid_input() {
        let input = quote! { invalid syntax };
        let result = domain_type_internal(input);
        let result_str = result.to_string();
        assert!(result_str.contains("compile_error"));
    }

    #[rstest]
    fn domain_type_internal_handles_valid_input() {
        let input = quote! {
            struct Valid(String);
        };
        let result = domain_type_internal(input);
        let result_str = result.to_string();
        assert!(result_str.contains("impl From < Valid > for String"));
    }
}
