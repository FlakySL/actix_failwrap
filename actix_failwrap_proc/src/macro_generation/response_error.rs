use proc_macro2::TokenStream as TokenStream2;

use crate::macro_input::response_error::MacroArgs;

pub fn response_error_macro(args: MacroArgs) -> TokenStream2 {
    let enum_name = args.name();

    args
        .branches()
        .iter()
        .map(|(ident, meta)| {
            
        })
}
