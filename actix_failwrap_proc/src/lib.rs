use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::macro_input::error_response::ErrorResponse;
use crate::macro_output::error_response::error_response;

mod helpers;
mod macro_input;
mod macro_output;


#[proc_macro_derive(ErrorResponse, attributes(default_status_code, status_code, transformer))]
pub fn error_response_macro(input: TokenStream) -> TokenStream {
    error_response(parse_macro_input!(input as ErrorResponse))
        .into()
}
