//! Author: irith
//! Date: 2025-11-17 @ 12:44pm
//! Description: Implementation of the `AutoIntoResponse` derive +
//! `auto_into_response` attribute macros. Here, we use `proc-macro2`.

use proc_macro_error::abort_call_site;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Attribute,
    Data,
    DataEnum,
    DeriveInput,
    Expr,
    LitBool,
    Token,
};
use syn::parse::{
    Parse,
    ParseStream,
};


const HELP: &str = "derive example: #[derive(AutoIntoResponse)]\nattribute signature: auto_into_response(status_code: http::StatusCode, is_transparent: bool)";
const ITEM_TYPE_ERROR: &str = "macro can only be derived on an enum or a struct";
const NO_ARGS_ERROR: &str = "no matching attribute found";


/// Definition for attribute "signature" (what params it takes).
#[derive(Debug)]
struct AutoIntoResponseArgs {
    status_code: Expr,
    is_transparent: LitBool,
}

impl Parse for AutoIntoResponseArgs {
    /// Parse the given `auto_into_response` helper attribute call
    /// (essentially as function arguments to the defined
    /// `auto_into_response` signature).
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let status_code = input.parse()?;
        let _: Token![,] = input.parse()?;
        let is_transparent = input.parse()?;

        Ok(Self {status_code, is_transparent})
    }
}


/// Build the auto-implementation for the `IntoResponse` trait.
pub fn impl_auto_into_response(ast: DeriveInput) -> TokenStream {
    // Get the name of the struct.
    let name = &ast.ident;

    // Ensure it's being auto-implementing on an enum or a struct.
    let status_code_logic = match ast.data {
        Data::Enum(e) => status_code_from_enum(e),
        Data::Struct(_) => status_code_from_attrs(&ast.attrs),
        _ => abort_call_site!(ITEM_TYPE_ERROR; help = HELP),
    };

    // Auto-implement the `IntoResponse` trait using the error string, the
    // corresponding logic for the given status code + transparency, and
    // `axum`'s `Json`.
    // TODO: grab HTTPErrorResponse from a separate location, or receive as arg.
    quote! {
        impl axum::response::IntoResponse for #name {
            fn into_response(self) -> axum::response::Response {
                let mut error = self.to_string();
                tracing::error!("{}", error);
                let status_code = #status_code_logic;
                let response = HTTPErrorResponse::new(error);
                (status_code, axum::extract::Json(response)).into_response()
            }
        }
    }
}


/// Generate the `match` and corresponding arms for an enum's variants.
/// The corresponding status codes per variant are determined by the
/// associated helper attribute.
fn status_code_from_enum(e: DataEnum) -> TokenStream {
    let arms = e.variants.iter()
                         .map(|v| {
                             // For each variant, create the corresponding
                             // pattern according to its definition.
                             let name = &v.ident;
                             let fields = match v.fields.len() {
                                0 => quote! {},
                                n => {
                                    let fields = vec![quote! {_}; n];
                                    quote! { (#(#fields),*) }
                                },
                             };

                             // Generate the status code logic and return
                             // the arm.
                             let status_code_logic = status_code_from_attrs(&v.attrs);
                             quote! {
                                  Self::#name #fields => #status_code_logic
                             }
                         })
                         .collect::<Vec<_>>();

    // Take all the match arms and return the whole match expression.
    quote! {
        match self {
            #(#arms),*
        }
    }
}


/// Generate the status code logic from the (first) associated helper
/// attribute.
fn status_code_from_attrs(attrs: &Vec<Attribute>) -> TokenStream {
    // Find the (first) matching associated helper attribute.
    let raw_args = attrs.iter().filter_map(helper_attribute_args).next();
    let args = match raw_args {
        // If not found, throw an error.
        None => abort_call_site!(NO_ARGS_ERROR; help = HELP),
        Some(a) => a,
    };

    // If we want to be transparent about the error we received, pass it
    // back directly, otherwise provide a generic reason based on the
    // status code.
    let status_code = args.status_code;
    let mut status_code_logic = quote! {
        #status_code
    };
    if !args.is_transparent.value {
        status_code_logic = quote! {{
            let status_code = #status_code_logic;
            error = status_code.canonical_reason()
                               .expect("StatusCode had no canonical reason?")
                               .to_string();
            status_code
        }};
    }

    status_code_logic
}


/// A filter-map function for finding + extracting an
/// `#[auto_into_response(status_code, is_transparent)]` attribute.
fn helper_attribute_args(a: &Attribute) -> Option<AutoIntoResponseArgs> {
    if !a.path().is_ident("auto_into_response") {
        return None
    }

    a.parse_args::<AutoIntoResponseArgs>().ok()
}
