#![deny(
    warnings,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

//! Helper for [concourse-resource] crate, to derive the `Vec<KV>` struct needed by [Concourse]
//! for metadata from any struct that `impl Serialize` from serde. Refer to [concourse-resource]
//! for usage.
//!
//! [Concourse]: https://concourse-ci.org
//! [concourse-resource]: https://github.com/mockersf/concourse-resource-rs

extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(IntoMetadataKV)]
pub fn metadata_kv_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation

    match ast.data {
        syn::Data::Struct(fields) => impl_metadata_kv(ast.ident, fields),
        _ => panic!("#[derive(IntoMetadataKV)] is only defined for structs"),
    }
}

fn impl_metadata_kv(name: syn::Ident, data_struct: syn::DataStruct) -> TokenStream {
    let md_fields: Vec<_> = data_struct
        .fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .map(|field_name| {
            quote! {
                concourse_resource::internal::KV {
                    name: String::from(stringify!(#field_name)),
                    value: serde_json::to_string(&self.#field_name).unwrap()
                }
            }
        })
        .collect();

    let gen = quote! {
        impl IntoMetadataKV for #name {
            fn into_metadata_kv(self) -> Vec<concourse_resource::internal::KV> {
                // let mut md = Vec::new();
                // md
                vec![#(#md_fields,)*]
            }
        }
    };
    gen.into()
}
