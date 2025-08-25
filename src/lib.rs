#![doc = include_str!("../README.md")]

// TODO: Check Clippy lints
// TODO: Document each test module for tests
// TODO: Add security mention in readme.md
#![deny(missing_docs)]

#![deny(warnings)]
#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_used)]

use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::macro_input::error_response::ErrorResponse;
use crate::macro_input::proof_route::{ProofRouteBody, ProofRouteMeta};
use crate::macro_output::error_response::error_response_output;
use crate::macro_output::proof_route::proof_route_output;

mod helpers;
mod macro_input;
mod macro_output;

#[cfg(test)]
mod tests;

/// # `ErrorResponse` Derive Macro
///
/// This macro is a helper to implement `Into<actix_web::HttpResponse>`
/// and `Into<actix_web::Error>` for `thiserror::error` marked enumerables.
///
/// Have in mind that nothing really enforces that the enum you apply this on
/// has also derived `thiserror::error`, but this macro's generation will rely
/// on you having implemented `Display`, and `thiserror::error` is a convenient
/// way to implement `Display`.
///
/// ## Macro attributes
///
/// With `ErrorResponse` you can modify how your your response will look when
/// returning an error from an endpoint.
///
/// **`#[transform_response(function_reference)]`**
/// You can add this attribute to your enum and pass a static function reference
/// the function should receive an `HttpResponseBuilder` which is the partially built
/// response and a `String` as second argument, which is the result of `<Self as Display>::to_string()`
/// where `Self` is the enum you applied this function to. The function should return an
/// `HttpResponse` which is what's going to be used when an error is returned from an endpoint.
///
/// **`#[default_status_code(number_or_identifier)]`**
/// You can add this attribute to your enum and pass or either a number representing
/// the http error status code like `400` or `500`, or an identifier such as `BadRequest`
/// or `InternalServerError`. This will set the status code by default if you don't
/// use `status_code` in any enum variant.
///
/// **`#[status_code(number_or_identifier)]`**
/// Like `default_status_code` you can pass a number or an HTTP status code identifier
/// and it will be applied to the current enum variant.
///
/// By default all status codes will be `InternalServerError` and the enum's Display
/// will be applied to the response body.
#[proc_macro_derive(
    ErrorResponse,
    attributes(default_status_code, status_code, transform_response)
)]
pub fn error_response(input: TokenStream) -> TokenStream {
    error_response_output(&parse_macro_input!(input as ErrorResponse)).into()
}

/// # `proof_route` Attribute Macro
///
/// You can replace the `actix_web::{get, post, put, ..}` macros by `proof_route`
/// with the following syntax `#[proof_route("METHOD /path")]` resembling to
/// the HTTP standard syntax.
///
/// **Before using this macro see [`ErrorResponse`] as you need it to use this**
///
/// This macro creates a new `actix_web` route, the syntax is the same as normal
/// attribute marked routes, except the return type changes to be a `Result<T, E>`
/// where `T` should implement `::actix_web::Responder` and `E` should implement
/// `Into<::actix_web::HttpResponse>` which you can implement in your response
/// type by using [`ErrorResponse`].
///
/// Since this thightly integrates with with `thiserror` you can join multiple
/// error types and use `?` to make your error handling in routes ergonomic.
///
/// If you return a custom error not annotated with [`ErrorResponse`] this is
/// considered undefined behavior, and no support will be given to that.
///
/// ## Macro Attributes
///
/// **`#[error_override(EnumVariant)]`**
///
/// You can also annotate your route collectors with the `error_override` attribute
/// which expects a variant of the error enumerable being returned. This will
/// replace any error that may be returned by the collector itself for a custom
/// error variant instead.
#[proc_macro_attribute]
pub fn proof_route(meta: TokenStream, body: TokenStream) -> TokenStream {
    proof_route_output(
        &parse_macro_input!(meta as ProofRouteMeta),
        &parse_macro_input!(body as ProofRouteBody),
    )
    .into()
}
