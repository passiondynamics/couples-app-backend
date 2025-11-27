//! Author: irith
//! Date: 2025-11-16 @ 10:36pm
//! Description: Implementation of the `AutoNew` derive macro. Here, we
//! use `proc-macro2`.

use proc_macro_error::abort_call_site;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Data,
    DeriveInput,
};


const HELP: &str = "derive example: #[derive(AutoNew)]";
const ITEM_TYPE_ERROR: &str = "macro can only be derived on a struct";


/// Build the auto-implementation for the `new` function.
pub fn impl_auto_new(ast: DeriveInput) -> TokenStream {
    // Get the name of the struct.
    let name = &ast.ident;

    // Ensure it's being auto-implementing on a struct.
    let struct_ast = match ast.data {
        Data::Struct(s) => s,
        _ => abort_call_site!(ITEM_TYPE_ERROR; help = HELP),
    };

    // Format the fields for params + args.
    let fields = struct_ast.fields;
    let params = fields.iter()
                       .map(|f| {
                           let ident = &f.ident;
                           let ty = &f.ty;
                           quote! { #ident: #ty }
                       })
                       .collect::<Vec<_>>();
    let idents = fields.iter()
                       .map(|f| {
                           let ident = &f.ident;
                           quote! { #ident }
                       })
                       .collect::<Vec<_>>();

    // Auto-implement the `new` function to pass the params as args to
    // struct creation.
    quote! {
        impl #name {
            pub fn new(#(#params),*) -> Self {
                Self {#(#idents),*}
            }
        }
    }
}
