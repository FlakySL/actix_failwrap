use macro_generation::response_error::response_error_macro;
use macro_input::response_error::MacroArgs as ResponseErrorMacroArgs;
use proc_macro::TokenStream;
use syn::parse_macro_input;


mod macro_generation;
mod macro_input;
mod misc;

#[proc_macro_derive(HelperAttr, attributes(failwrap, response))]
pub fn response_error(attr: TokenStream) -> TokenStream {
    response_error_macro(parse_macro_input!(attr as ResponseErrorMacroArgs)).into()
}
