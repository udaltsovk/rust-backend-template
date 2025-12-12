use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{ItemImpl, LitStr};

pub fn instrument_all(attr: TokenStream, stream: TokenStream) -> TokenStream {
    instrument_all_internal(attr.into(), stream.into()).into()
}

fn instrument_all_internal(
    attr: TokenStream2,
    stream: TokenStream2,
) -> TokenStream2 {
    let repo_name = match syn::parse2::<LitStr>(attr) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error(),
    };
    let impl_block = match syn::parse2::<ItemImpl>(stream) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error(),
    };
    instrument_all_impl(&repo_name, impl_block)
}

pub fn instrument_all_impl(
    repo_name: &LitStr,
    mut impl_block: ItemImpl,
) -> TokenStream2 {
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

    quote!(#impl_block)
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use rstest::rstest;
    use syn::{ItemImpl, LitStr, parse_quote};

    use super::{instrument_all_impl, instrument_all_internal};

    #[rstest]
    fn instrument_all_basic_impl() {
        let repo_name: LitStr =
            syn::parse_str("\"UserRepository\"").expect("Valid LitStr");
        let item: ItemImpl = parse_quote! {
            impl UserRepository {
                fn find_by_id(&self, id: UserId) -> Option<User> {
                    // implementation
                }
            }
        };

        let result_tokens = instrument_all_impl(&repo_name, item);
        let result_str = result_tokens.to_string();

        assert!(result_str.contains("# [tracing :: instrument (name = \"UserRepository::find_by_id\" , skip_all)]"));
        assert!(result_str.contains("fn find_by_id"));
    }

    #[rstest]
    fn instrument_all_multiple_methods() {
        let repo_name: LitStr =
            syn::parse_str("\"ProductService\"").expect("Valid LitStr");
        let item: ItemImpl = parse_quote! {
            impl ProductService {
                fn create_product(&self, data: ProductData) -> Result<Product, Error> {
                    // implementation
                }

                fn get_product(&self, id: &ProductId) -> Option<Product> {
                    // implementation
                }

                fn delete_product(&mut self, id: ProductId) -> bool {
                    // implementation
                }
            }
        };

        let result_tokens = instrument_all_impl(&repo_name, item);
        let result_str = result_tokens.to_string();

        assert!(result_str.contains("# [tracing :: instrument (name = \"ProductService::create_product\" , skip_all)]"));
        assert!(result_str.contains("# [tracing :: instrument (name = \"ProductService::get_product\" , skip_all)]"));
        assert!(result_str.contains("# [tracing :: instrument (name = \"ProductService::delete_product\" , skip_all)]"));
    }

    #[rstest]
    fn instrument_all_empty_impl() {
        let repo_name: LitStr =
            syn::parse_str("\"EmptyService\"").expect("Valid LitStr");
        let item: ItemImpl = parse_quote! {
            impl EmptyService {
            }
        };

        let result_tokens = instrument_all_impl(&repo_name, item);
        let result_str = result_tokens.to_string();

        assert!(result_str.contains("impl EmptyService"));
        // Should not contain any tracing attributes since there are no methods
        assert!(!result_str.contains("tracing :: instrument"));
    }

    #[rstest]
    fn instrument_all_preserves_existing_attributes() {
        let repo_name: LitStr =
            syn::parse_str("\"TestService\"").expect("Valid LitStr");
        let item: ItemImpl = parse_quote! {
            impl TestService {
                #[allow(dead_code)]
                #[cfg(test)]
                fn test_method(&self) -> bool {
                    true
                }
            }
        };

        let result_tokens = instrument_all_impl(&repo_name, item);
        let result_str = result_tokens.to_string();

        assert!(result_str.contains("# [allow (dead_code)]"));
        assert!(result_str.contains("# [cfg (test)]"));
        assert!(result_str.contains("# [tracing :: instrument (name = \"TestService::test_method\" , skip_all)]"));
    }

    #[rstest]
    fn instrument_all_with_generics() {
        let repo_name: LitStr =
            syn::parse_str("\"Repository\"").expect("Valid LitStr");
        let item: ItemImpl = parse_quote! {
            impl<T> Repository<T> where T: Clone {
                fn find(&self, id: &str) -> Option<T> {
                    // implementation
                }

                fn save(&mut self, entity: T) -> Result<(), Error> {
                    // implementation
                }
            }
        };

        let result_tokens = instrument_all_impl(&repo_name, item);
        let result_str = result_tokens.to_string();

        assert!(result_str.contains(
            "# [tracing :: instrument (name = \"Repository::find\" , skip_all)]"
        ));
        assert!(result_str.contains(
            "# [tracing :: instrument (name = \"Repository::save\" , skip_all)]"
        ));
        assert!(
            result_str.contains("impl < T > Repository < T > where T : Clone")
        );
    }

    #[rstest]
    fn instrument_all_async_methods() {
        let repo_name: LitStr =
            syn::parse_str("\"AsyncService\"").expect("Valid LitStr");
        let item: ItemImpl = parse_quote! {
            impl AsyncService {
                async fn fetch_data(&self) -> Result<Data, Error> {
                    // async implementation
                }

                async fn process_data(&mut self, data: Data) -> ProcessedData {
                    // async implementation
                }
            }
        };

        let result_tokens = instrument_all_impl(&repo_name, item);
        let result_str = result_tokens.to_string();

        assert!(result_str.contains("# [tracing :: instrument (name = \"AsyncService::fetch_data\" , skip_all)]"));
        assert!(result_str.contains("# [tracing :: instrument (name = \"AsyncService::process_data\" , skip_all)]"));
        assert!(result_str.contains("async fn fetch_data"));
        assert!(result_str.contains("async fn process_data"));
    }

    #[rstest]
    fn instrument_all_special_characters_in_name() {
        let repo_name: LitStr = syn::parse_str("\"My-Special_Repository123\"")
            .expect("Valid LitStr");
        let item: ItemImpl = parse_quote! {
            impl MyRepository {
                fn do_something(&self) {
                    // implementation
                }
            }
        };

        let result_tokens = instrument_all_impl(&repo_name, item);
        let result_str = result_tokens.to_string();

        assert!(result_str.contains("# [tracing :: instrument (name = \"My-Special_Repository123::do_something\" , skip_all)]"));
    }

    #[rstest]
    fn instrument_all_complex_method_signatures() {
        let repo_name: LitStr =
            syn::parse_str("\"ComplexService\"").expect("Valid LitStr");
        let item: ItemImpl = parse_quote! {
            impl ComplexService {
                async fn complex_method<T, U>(&mut self, param1: T, param2: &U) -> Result<Vec<T>, Box<dyn std::error::Error>>
                where
                    T: Clone + Send + Sync,
                    U: std::fmt::Debug,
                {
                    // complex implementation
                }
            }
        };

        let result_tokens = instrument_all_impl(&repo_name, item);
        let result_str = result_tokens.to_string();

        assert!(result_str.contains("# [tracing :: instrument (name = \"ComplexService::complex_method\" , skip_all)]"));
        assert!(result_str.contains("async fn complex_method"));
    }

    #[rstest]
    fn instrument_all_with_trait_impl() {
        let repo_name: LitStr =
            syn::parse_str("\"TraitImpl\"").expect("Valid LitStr");
        let item: ItemImpl = parse_quote! {
            impl SomeTrait for MyStruct {
                fn trait_method(&self) -> String {
                    // trait implementation
                }

                fn another_trait_method(&mut self, value: i32) {
                    // trait implementation
                }
            }
        };

        let result_tokens = instrument_all_impl(&repo_name, item);
        let result_str = result_tokens.to_string();

        assert!(result_str.contains("# [tracing :: instrument (name = \"TraitImpl::trait_method\" , skip_all)]"));
        assert!(result_str.contains("# [tracing :: instrument (name = \"TraitImpl::another_trait_method\" , skip_all)]"));
        assert!(result_str.contains("impl SomeTrait for MyStruct"));
    }

    #[rstest]
    fn instrument_all_preserves_method_body() {
        let repo_name: LitStr =
            syn::parse_str("\"TestRepo\"").expect("Valid LitStr");
        let item: ItemImpl = parse_quote! {
            impl TestRepo {
                fn get_value(&self) -> i32 {
                    let x = 42;
                    let y = x * 2;
                    y + 10
                }
            }
        };

        let result_tokens = instrument_all_impl(&repo_name, item);
        let result_str = result_tokens.to_string();

        assert!(result_str.contains("# [tracing :: instrument (name = \"TestRepo::get_value\" , skip_all)]"));
        assert!(result_str.contains("let x = 42"));
        assert!(result_str.contains("let y = x * 2"));
        assert!(result_str.contains("y + 10"));
    }

    #[rstest]
    fn instrument_all_with_associated_items() {
        let repo_name: LitStr =
            syn::parse_str("\"CompleteService\"").expect("Valid LitStr");
        let item: ItemImpl = parse_quote! {
            impl CompleteService {
                const MAX_SIZE: usize = 1000;
                type ResultType = Result<String, Error>;

                fn process(&self) -> Self::ResultType {
                    // implementation
                }
            }
        };

        let result_tokens = instrument_all_impl(&repo_name, item);
        let result_str = result_tokens.to_string();

        // Should only instrument functions, not constants or types
        assert!(result_str.contains("# [tracing :: instrument (name = \"CompleteService::process\" , skip_all)]"));
        assert!(result_str.contains("const MAX_SIZE : usize = 1000"));
        assert!(
            result_str.contains("type ResultType = Result < String , Error >")
        );
        // Should not instrument non-function items
        assert!(
            !result_str
                .contains("instrument (name = \"CompleteService::MAX_SIZE\"")
        );
        assert!(
            !result_str
                .contains("instrument (name = \"CompleteService::ResultType\"")
        );
    }

    // #[rstest]
    // fn instrument_all_unicode_repo_name() {
    //     let repo_name: LitStr = syn::parse_str("\"Репозиторий\"").expect("Valid LitStr");
    //     let item: ItemImpl = parse_quote! {
    //         impl UnicodeRepo {
    //             #[expect(
    //                 clippy::disallowed_script_idents,
    //                 reason = "we have to use cyrillic for testing purposes"
    //             )]
    //             fn метод(&self) -> String {
    //                 // implementation
    //             }
    //         }
    //     };

    //     let result_tokens = instrument_all_impl(&repo_name, item);
    //     let result_str = result_tokens.to_string();

    //     assert!(result_str.contains("# [tracing :: instrument (name = \"Репозиторий::метод\" , skip_all)]"));
    // }

    #[rstest]
    fn instrument_all_empty_string_repo_name() {
        let repo_name: LitStr = syn::parse_str("\"\"").expect("Valid LitStr");
        let item: ItemImpl = parse_quote! {
            impl EmptyNameRepo {
                fn method(&self) -> String {
                    // implementation
                }
            }
        };

        let result_tokens = instrument_all_impl(&repo_name, item);
        let result_str = result_tokens.to_string();

        assert!(result_str.contains(
            "# [tracing :: instrument (name = \"::method\" , skip_all)]"
        ));
    }

    #[rstest]
    fn instrument_all_preserves_visibility() {
        let repo_name: LitStr =
            syn::parse_str("\"VisibilityTest\"").expect("Valid LitStr");
        let item: ItemImpl = parse_quote! {
            impl VisibilityTest {
                pub fn public_method(&self) -> String {
                    // implementation
                }

                pub(crate) fn crate_method(&self) -> String {
                    // implementation
                }

                fn private_method(&self) -> String {
                    // implementation
                }
            }
        };

        let result_tokens = instrument_all_impl(&repo_name, item);
        let result_str = result_tokens.to_string();

        assert!(result_str.contains("pub fn public_method"));
        assert!(result_str.contains("pub (crate) fn crate_method"));
        assert!(result_str.contains("# [tracing :: instrument (name = \"VisibilityTest::public_method\" , skip_all)]"));
        assert!(result_str.contains("# [tracing :: instrument (name = \"VisibilityTest::crate_method\" , skip_all)]"));
        assert!(result_str.contains("# [tracing :: instrument (name = \"VisibilityTest::private_method\" , skip_all)]"));
    }

    #[rstest]
    fn instrument_all_with_unsafe_methods() {
        let repo_name: LitStr =
            syn::parse_str("\"UnsafeService\"").expect("Valid LitStr");
        let item: ItemImpl = parse_quote! {
            impl UnsafeService {
                unsafe fn unsafe_method(&self, ptr: *mut u8) -> *const u8 {
                    // unsafe implementation
                }

                fn safe_method(&self) -> String {
                    // safe implementation
                }
            }
        };

        let result_tokens = instrument_all_impl(&repo_name, item);
        let result_str = result_tokens.to_string();

        assert!(result_str.contains("# [tracing :: instrument (name = \"UnsafeService::unsafe_method\" , skip_all)]"));
        assert!(result_str.contains("# [tracing :: instrument (name = \"UnsafeService::unsafe_method\" , skip_all)]"));
        assert!(result_str.contains("unsafe fn unsafe_method"));
    }

    #[rstest]
    fn instrument_all_internal_handles_invalid_input() {
        let attr = quote! { invalid };
        let stream = quote! { impl Foo {} };

        let result = instrument_all_internal(attr, stream);
        let result_str = result.to_string();

        assert!(result_str.contains("compile_error"));
    }

    #[rstest]
    fn instrument_all_internal_handles_invalid_impl() {
        let attr = quote! { "Repo" };
        let stream = quote! { invalid };

        let result = instrument_all_internal(attr, stream);
        let result_str = result.to_string();

        assert!(result_str.contains("compile_error"));
    }

    #[rstest]
    fn instrument_all_internal_handles_valid_input() {
        let attr = quote! { "Repo" };
        let stream = quote! { impl Foo { fn bar(&self) {} } };

        let result = instrument_all_internal(attr, stream);
        let result_str = result.to_string();

        assert!(result_str.contains("tracing :: instrument"));
    }
}
