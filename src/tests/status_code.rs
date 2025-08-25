//! [`StatusCode`] Parsing Tests
//!
//! This tests check whether the [`StatusCode`]
//! meta structure parses correctly.

use quote::quote;
use syn::parse2;

use crate::macro_input::error_response::StatusCode;

/// Only Valid Code Identifiers
///
/// This tests whether invalid identifiers such as non usize values
/// trigger errors for [`StatusCode`].
#[test]
pub fn only_valid_status_code_identifiers() {
    parse2::<StatusCode>(quote! { 9999999999999999999999999999999999999999999999999999999999999 })
        .expect_err("Expected error invalid usize value.");
}

/// Only Error Statuses Allowed
///
/// This tests whether non error statuses trigger an error
/// in any part of the crate.
#[test]
pub fn only_error_status_code_allowed() {
    parse2::<StatusCode>(quote! { 200 }).expect_err("Expected error only error http status codes.");
}

/// Invalid Status Identifiers
///
/// This tests whether an invalid identifier for a status
/// code triggers an error.
#[test]
pub fn valid_ident_invalid_status() {
    parse2::<StatusCode>(quote! { Goen }).expect_err("Expected error required a valid identifier");

    parse2::<StatusCode>(quote! { Gone })
        .expect("Expected to be able to parse value as Gone http status code");
}

/// Invalid Everytyhing
///
/// This tests whether not passing a number nor an ident
/// to something expecting a [`StatusCode`] triggers an error.
#[test]
pub fn invalid_everything() {
    parse2::<StatusCode>(quote! { ()1dsa23... }).expect_err("Expected a valid identifier");
}
