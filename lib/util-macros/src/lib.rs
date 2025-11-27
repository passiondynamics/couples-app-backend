//! Author: irith
//! Date: 2025-11-12 @ 1:05pm
//! Description: Macros for simplifying boilerplate in models. We choose
//! to use `proc_macro` in this top-level file, and `proc_macro2` in the
//! lower level implementation files (having two libs for the same thing
//! is very confusing ik, trust me, see
//! [here](https://petanode.com/posts/rust-proc-macro/) for the
//! inspiration for this structure).

use proc_macro::TokenStream;
use proc_macro_error::{
    proc_macro_error,
};
use syn::parse_macro_input;

mod auto_deserialize;
use auto_deserialize::impl_auto_deserialize;
mod auto_into_response;
use auto_into_response::impl_auto_into_response;
mod auto_new;
use auto_new::impl_auto_new;


/// Automatically implement the `new` function for the given item, taking
/// the fields of the item as params and returning a new struct with those
/// fields passed through. (save 10 min of boilerplate writing by 3 hours
/// of macro writing, for the fun/challenge of it). Inspired by
/// https://ferrous-systems.com/blog/testing-proc-macros/.
///
/// Typically this would be done under the struct with something like, for
/// example with `User`:
/// ```
/// impl User {
///     pub fn new(id: i64, username: Username, password: Password, preferences: UserPreferences) -> Self {
///         Self { id, username, password, preferences }
///     }
/// }
/// ```
#[proc_macro_derive(AutoNew)]
#[proc_macro_error]
pub fn auto_new_derive(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item);
    impl_auto_new(ast).into()
}


/// Auto-implement the `IntoResponse` trait for a given error struct.
///
/// Typically something like, for `InvalidUsernameError`:
/// ```
/// impl IntoResponse for InvalidUsernameError {
///     fn into_response(self) -> Response {
///         let error = self.to_string();
///         error!("{}", error);
///         let status_code = StatusCode::UNPROCESSABLE_ENTITY;
///         let response = HTTPErrorResponse::new(error);
///         (status_code, Json(response)).into_response()
///     }
/// }
/// ```
#[proc_macro_derive(AutoIntoResponse, attributes(auto_into_response))]
#[proc_macro_error]
pub fn auto_into_response(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item);
    impl_auto_into_response(ast).into()
}


/// Auto-implement a `Raw{name}` struct for a given (/final) struct. This new
/// intermediate struct has identical fields, except the types used are
/// the base types for those fields. In addition, auto-implement a
/// conversion method into a value of the final struct/type.
///
/// Typically something like, for `AddUserRequest`:
/// ```
/// #[derive(Debug, Deserialize)]
/// pub struct RawAddUserRequest {
///     username: String,
///     password: String,
/// }
/// 
/// impl TryFrom<RawAddUserRequest> for AddUserRequest {
///     type Error = ErrorResponse;
///     fn try_from(raw: RawAddUserRequest) -> Result<Self, Self::Error> {
///         let username = Username::new(&raw.username)?;
///         let password = Password::new(&raw.password)?;
///         Ok(Self::new(username, password))
///     }
/// }
/// ```
///
/// ---
///
/// Look. I know. It's the same thing, basically written twice. Trust me,
/// I hate it too. Let me explain.
/// - To go from the raw body given by `axum` into our struct, we need to
/// deserialize with `serde`.
/// - The problem is, `serde` does not make it easy to deserialize and
/// keep the custom error we return ourselves (and correspondingly, the
/// IntoResponse method), only their own error type.
/// - So we unfortunately need to break it down into two separate steps:
///   - raw body -> the deserialized structure, but with raw types,
///   - the deserialized structure -> the actual structure with the
///   correct types.
/// So we have to have a duplicate struct when we have validations, until
/// we figure a way around this.
#[proc_macro_derive(AutoDeserialize, attributes(auto_deserialize))]
#[proc_macro_error]
pub fn auto_deserialize(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item);
    impl_auto_deserialize(ast).into()
}
