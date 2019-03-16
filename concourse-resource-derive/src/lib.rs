extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

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
                concourse_resource::KV {
                    name: String::from(stringify!(#field_name)),
                    value: serde_json::to_string(&self.#field_name).unwrap()
                }
            }
        })
        .collect();

    let gen = quote! {
        impl IntoMetadataKV for #name {
            fn into_metadata_kv(self) -> Vec<KV> {
                // let mut md = Vec::new();
                // md
                vec![#(#md_fields,)*]
            }
        }
    };
    gen.into()
}
