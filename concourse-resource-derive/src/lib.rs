extern crate proc_macro;

use crate::proc_macro::TokenStream;
// use quote::quote;
use syn;

#[proc_macro_derive(IntoMetadataKV)]
pub fn metadata_kv_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_metadata_kv(&ast)
}

fn impl_metadata_kv(_ast: &syn::DeriveInput) -> TokenStream {
    unimplemented!()
}
