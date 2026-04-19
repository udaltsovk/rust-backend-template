use std::path::{Path, PathBuf};

use proc_macro2::Span;
use quote::quote;
use syn::{
    Expr, LitStr, Path as SynPath, Result, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

fn file_stem(path: &Path) -> Result<&str> {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| {
            syn::Error::new(
                Span::call_site(),
                "could not determine invocation file name",
            )
        })
}

fn file_name(path: &Path) -> Result<&str> {
    path.file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            syn::Error::new(
                Span::call_site(),
                "could not determine invocation file name",
            )
        })
}

fn postgres_dir_from_invocation(
    invocation_file: &Path,
) -> Result<PathBuf> {
    let path =
        invocation_file.parent().unwrap_or(invocation_file);

    if file_stem(invocation_file)? == "postgres" {
        let postgres_dir = path.join("postgres");

        if postgres_dir.is_dir() {
            return Ok(postgres_dir);
        }
    }

    path.ancestors()
        .find(|ancestor| {
            ancestor
                .file_name()
                .is_some_and(|part| part == "postgres")
        })
        .map(Path::to_path_buf)
        .ok_or_else(|| {
            syn::Error::new(
                Span::call_site(),
                "postgres SQL macros must be called from \
                 inside a `postgres` module tree",
            )
        })
}

fn path_from_src(path: &Path) -> Result<String> {
    let components = path
        .components()
        .skip_while(|component| {
            component.as_os_str() != "src"
        })
        .map(|component| {
            component.as_os_str().to_string_lossy()
        })
        .collect::<Vec<_>>();

    if components.is_empty() {
        return Err(syn::Error::new(
            Span::call_site(),
            "could not derive a crate-relative path \
             starting at `src`",
        ));
    }

    Ok(components.join("/"))
}

fn query_dir_from_invocation(
    invocation_file: &Path,
    postgres_dir: &Path,
) -> Result<PathBuf> {
    let file_name = file_name(invocation_file)?;
    let file_stem = file_stem(invocation_file)?;
    let parent_name = invocation_file
        .parent()
        .and_then(Path::file_name)
        .and_then(|name| name.to_str());

    if file_name == "repository.rs"
        || (file_name == "mod.rs"
            && parent_name == Some("repository"))
    {
        return Ok(postgres_dir.join("query"));
    }

    if parent_name == Some("repository") {
        return Ok(postgres_dir
            .join("query")
            .join(file_stem));
    }

    Ok(postgres_dir.join("query"))
}

pub fn feature_postgres_migrator2(
    invocation_file: &Path,
    input: &proc_macro2::TokenStream,
) -> Result<proc_macro2::TokenStream> {
    if !input.is_empty() {
        return Err(syn::Error::new(
            Span::call_site(),
            "migrate! takes no arguments",
        ));
    }

    let postgres_dir =
        postgres_dir_from_invocation(invocation_file)?;
    let migration_dir = postgres_dir.join("migration");

    let path = LitStr::new(
        &format!("./{}", path_from_src(&migration_dir)?),
        Span::call_site(),
    );

    Ok(quote! {
        sqlx::migrate!(#path)
    })
}

struct FeaturePostgresQueryFileAsInput {
    out_struct: SynPath,
    file: LitStr,
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for FeaturePostgresQueryFileAsInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let out_struct = input.parse()?;
        let _: Token![,] = input.parse()?;
        let file = input.parse()?;
        let args = if input.is_empty() {
            Punctuated::new()
        } else {
            let _: Token![,] = input.parse()?;
            Punctuated::parse_terminated(input)?
        };

        Ok(Self {
            out_struct,
            file,
            args,
        })
    }
}

pub fn feature_postgres_query_file_as2(
    invocation_file: &Path,
    input: proc_macro2::TokenStream,
) -> Result<proc_macro2::TokenStream> {
    let FeaturePostgresQueryFileAsInput {
        out_struct,
        file,
        args,
    } = syn::parse2(input)?;

    let postgres_dir =
        postgres_dir_from_invocation(invocation_file)?;
    let query_dir = query_dir_from_invocation(
        invocation_file,
        &postgres_dir,
    )?;
    let query_path = query_dir.join(file.value());

    let path = LitStr::new(
        &path_from_src(&query_path)?,
        Span::call_site(),
    );

    let args = args.into_iter().collect::<Vec<_>>();

    Ok(quote! {
        sqlx::query_file_as!(
            #out_struct,
            #path
            #(, #args)*
        )
    })
}
