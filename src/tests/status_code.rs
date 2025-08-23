use syn::parse2;
use quote::quote;

use crate::macro_input::error_response::StatusCode;

#[test]
pub fn only_valid_status_code_identifiers() {
    parse2::<StatusCode>(quote! { 9999999999999999999999999999999999999999999999999999999999999 })
        .expect_err("Expected error invalid usize value.");
}

#[test]
pub fn only_error_status_code_allowed() {
    parse2::<StatusCode>(quote! { 200 })
        .expect_err("Expected error only error http status codes.");
}

#[test]
pub fn valid_ident_invalid_status() {
    parse2::<StatusCode>(quote! { Goen })
        .expect_err("Expected error required a valid identifier");

    parse2::<StatusCode>(quote! { Gone })
        .expect("Expected to be able to parse value as Gone http status code");
}

#[test]
pub fn invalid_everything() {
    parse2::<StatusCode>(quote! { ()1dsa23... })
        .expect_err("Expected a valid identifier");
}
