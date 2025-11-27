//! Author: irith
//! Date: 2025-11-26 @ 4:46pm
//! Description: Implementation of the `AutoDeserialize` derive +
//! `auto_deserialize` attribute macros. Here, we use `proc-macro2`.

use proc_macro_error::abort_call_site;
use proc_macro2::TokenStream;
use quote::{
    format_ident,
    quote,
};
use syn::{
    Attribute,
    Data,
    DeriveInput,
    Field,
    Fields,
    Ident,
    Type,
};
use syn::parse::{
    Parse,
    ParseStream,
};


const HELP: &str = "derive example: #[derive(AutoDeserialize)]\nattribute signature: auto_deserialize(from_type: Type)";
const ITEM_TYPE_ERROR: &str = "macro can only be derived on a struct with named fields";
const NO_ARGS_ERROR: &str = "no matching attribute found";


/// Definition for attribute "signature" (what params it takes).
#[derive(Debug)]
struct AutoDeserializeArgs {
    from_type: Type,
}

impl Parse for AutoDeserializeArgs {
    /// Parse the given `auto_deserialize` helper attribute call
    /// (essentially as function arguments to the defined
    /// `auto_deserialize` signature).
    fn parse(input: ParseStream) -> syn::Result<Self> { // Explicitly label as `syn::Result` to make it clear it's not the standard `Result`.
        let from_type = input.parse()?;

        Ok(Self {from_type})
    }
}


#[derive(Debug)]
struct FieldSnippets {
    name: Ident,
    from_type_def: TokenStream,
    into_type_init: TokenStream,
}


/// Build the auto-implementation for the intermediate struct to
/// deserialize into and the `TryFrom` to convert to the final struct.
pub fn impl_auto_deserialize(ast: DeriveInput) -> TokenStream {
    // Get the name of the struct.
    let name = &ast.ident;

    // Ensure it's being auto-implemented on a struct with named fields.
    let raw_fields = match ast.data {
        Data::Struct(s) => s.fields,
        _ => abort_call_site!(ITEM_TYPE_ERROR; help = HELP),
    };

    let fields = match raw_fields {
        Fields::Named(f) => f.named,
        _ => abort_call_site!(ITEM_TYPE_ERROR; help = HELP),
    };

    // Generate the identifiers, struct field definitions, and final type
    // initializations, using the data from the final struct.
    let intermediate_name = format_ident!("Raw{}", name);
    let mut names = vec![];
    let mut from_type_defs = vec![];
    let mut into_type_inits = vec![];
    fields.iter()
          .map(build_field_snippets)
          .for_each(|fs| {
              names.push(fs.name);
              from_type_defs.push(fs.from_type_def);
              into_type_inits.push(fs.into_type_init);
          });

    // Auto-implement the intermediate struct and the corresponding logic
    // to convert into the final struct.
    quote! {
        #[derive(Debug, serde::Deserialize)]
        pub struct #intermediate_name {
            #(#from_type_defs),*
        }

        impl TryFrom<#intermediate_name> for #name {
            type Error = axum::response::ErrorResponse;
            fn try_from(raw: #intermediate_name) -> Result<Self, Self::Error> {
                #(#into_type_inits)*
                Ok(Self::new(#(#names),*))
            }
        }
    }
}


/// Given a field from the final struct, generate the corresponding code
/// snippets for that field:
/// - the field name,
/// - a field definition in the intermediate struct,
/// - an initialization of the value in its final type using the
/// intermediate value,
fn build_field_snippets(f: &Field) -> FieldSnippets {
    // Grab the three pieces of info needed.
    // If not found, throw an error.
    let name = f.ident.clone()
                      .unwrap_or_else(|| abort_call_site!(ITEM_TYPE_ERROR; help = HELP));

    let from_type = f.attrs.iter()
                           .filter_map(helper_attribute_args)
                           .next()
                           .unwrap_or_else(|| abort_call_site!(NO_ARGS_ERROR; help = HELP))
                           .from_type;

    let into_type = f.ty.clone();

    // Build out corresponding code snippets.
    let from_type_def = quote! {
        #name: #from_type
    };
    let into_type_init = quote! {
        let #name = #into_type::new(&raw.#name)?;
    };

    FieldSnippets {name, from_type_def, into_type_init}
}


/// A filter-map function for finding + extracting an
/// `#[auto_deserialize(type)]` attribute.
fn helper_attribute_args(a: &Attribute) -> Option<AutoDeserializeArgs> {
    if !a.path().is_ident("auto_deserialize") {
        return None
    }

    a.parse_args::<AutoDeserializeArgs>().ok()
}
