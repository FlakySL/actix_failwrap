//! [`ErrorResponse`] Tests
//!
//! This sub-module tests the [`ErrorResponse`] macro
//! parsing and generation, this is unitary testing, so
//! if you want to find integration testing for this macro
//! look for a `tests` folder in the crate root.

use quote::quote;
use syn::parse2;

use crate::ErrorResponse;

/// No Duplicated Attributes in [`ErrorResponse`]
///
/// This tests whether the attribute duplication checker
/// works for [`ErrorResponse`].
///
/// XXX: This test may be redundant / remove in favor of `unique_attr` tests.
#[test]
pub fn parse_error_no_duplicated_attributes() {
    parse2::<ErrorResponse>(quote! {
        #[default_status_code(InternalServerError)]
        #[default_status_code(500)]
        enum Error { X }
    })
    .expect_err("Expected error duplicated default_status_code attribute.");

    parse2::<ErrorResponse>(quote! {
        enum Error {
            #[status_code(InternalServerError)]
            #[status_code(500)]
            X
        }
    })
    .expect_err("Expected error duplicated status_code attribute.");
}

/// Find At Least One Variant In [`ErrorResponse`]
///
/// This tests whether an empty invocation for [`ErrorResponse`]
/// fails.
#[test]
pub fn parse_error_at_least_one_variant() {
    parse2::<ErrorResponse>(quote! {
        enum Error { }
    })
    .expect_err("Expected error required at least one variant.");
}
