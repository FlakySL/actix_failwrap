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

#[proc_macro_derive(
    ErrorResponse,
    attributes(default_status_code, status_code, transform_response)
)]
pub fn error_response(input: TokenStream) -> TokenStream {
    error_response_output(parse_macro_input!(input as ErrorResponse)).into()
}

#[proc_macro_attribute]
pub fn proof_route(meta: TokenStream, body: TokenStream) -> TokenStream {
    proof_route_output(
        parse_macro_input!(meta as ProofRouteMeta),
        parse_macro_input!(body as ProofRouteBody),
    )
    .into()
}
