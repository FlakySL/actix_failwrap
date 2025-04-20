use std::fmt::Display;

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::Error as SynError;

pub trait IntoCompileError
where
    Self: Display + Sized
{
    #[cold]
    fn to_compile_error(&self) -> TokenStream2 {
        let message = self.to_string();
        quote! { std::compile_error!(#message) }
    }

    #[cold]
    fn to_syn_error<T: ToTokens>(&self, span: T) -> SynError {
        SynError::new_spanned(span, self.to_string())
    }
}

impl<T: Display> IntoCompileError for T {}

macro_rules! handle_macro_result {
    ($val:expr) => {{
        use $crate::macros::errors::IntoCompileError;

        match $val {
            std::result::Result::Ok(value) => value,
            std::result::Result::Err(error) => return error.to_compile_error(),
        }
    }};
}

#[allow(unused_imports)]
pub(crate) use handle_macro_result;
