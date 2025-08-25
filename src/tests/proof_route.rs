//! [`proof_route`] Tests
//!
//! This sub-module tests the [`proof_route`] macro
//! parsing and generation, this is unitary testing, so
//! if you want to find integration testing for this macro
//! look for a `tests` folder in the crate root.
//!
//! [`proof_route`]: crate::proof_route

use quote::quote;
use syn::parse2;

use crate::{ProofRouteBody, ProofRouteMeta};

/// No Space In Between Method And Route In Meta
///
/// This tests whether the route meta errors in the case
/// there is no space in between the method and the path.
#[test]
pub fn proof_route_meta_no_space() {
    parse2::<ProofRouteMeta>(quote! { "GET/route/test" })
        .expect_err("Expected error no space between route meta.");
}

/// Invalid HTTP Method In Meta
///
/// This tests whether an invalid HTTP method triggers
/// an error in the route meta.
#[test]
pub fn proof_route_meta_invalid_http_method() {
    parse2::<ProofRouteMeta>(quote! { "BADMETHOD /route/test" })
        .expect_err("Expected error no space between route meta.");
}

/// Actix Attributes Not Allowed In Handler
///
/// This tests whether adding a base `actix_web` attribute
/// triggers an error, as this would conflict with the wrapper.
#[test]
pub fn proof_route_body_actix_attributes_not_allowed() {
    parse2::<ProofRouteBody>(quote! {
        #[get("/")]
        async fn x() {}
    })
    .expect_err("Expected error no actix attributes allowed.");
}

/// No Unit Type Return In [`proof_route`]
///
/// This tests whether declaring no return type i.e
/// unit type or () in a handler triggers an error.
///
/// [`proof_route`]: crate::proof_route
#[test]
pub fn proof_route_no_unit_type() {
    parse2::<ProofRouteBody>(quote! {
        async fn x() { () }
    })
    .expect_err("Expected error no unit type allowed as return.");
}

/// Only Result As Return Type In [`proof_route`]
///
/// This tests many variants that should fail if attempted
/// to be returned on a [`proof_route`] handler.
///
/// [`proof_route`]: crate::proof_route
#[test]
pub fn proof_route_only_result() {
    parse2::<ProofRouteBody>(quote! {
        async fn x() -> Result<_, _> {
            Ok(HttpResponse::Ok().finish())
        }
    })
    .expect("Expected success as the return type is valid.");

    parse2::<ProofRouteBody>(quote! {
        async fn x() -> Result<HttpResponse, Error> {
            Ok(HttpResponse::Ok().finish())
        }
    })
    .expect("Expected success as the return type is valid.");

    parse2::<ProofRouteBody>(quote! {
        async fn x() -> &Result<_, _> {
            Ok(HttpResponse::Ok().finish())
        }
    })
    .expect_err("Expected error as a reference is returned.");

    parse2::<ProofRouteBody>(quote! {
        async fn x() -> BlahBlah<_, _> {
            Ok(HttpResponse::Ok().finish())
        }
    })
    .expect_err("Expected error because a Result is not returned.");
}

/// Invalid Generic Semantics For Return Type In [`proof_route`]
///
/// This tests whether the generic type semantics are correct
/// for the return type in a [`proof_route`] handler.
///
/// [`proof_route`]: crate::proof_route
#[test]
pub fn proof_route_invalid_return_generics() {
    parse2::<ProofRouteBody>(quote! {
        async fn x() -> Result<_> {
            Ok(HttpResponse::Ok().finish())
        }
    })
    .expect_err("Expected error because an invalid Result is returned.");

    parse2::<ProofRouteBody>(quote! {
        async fn x() -> Result {
            Ok(HttpResponse::Ok().finish())
        }
    })
    .expect_err("Expected error because the returned Result doesn't have any arguments.");

    parse2::<ProofRouteBody>(quote! {
        async fn x() -> Result<'a, Error> {
            Ok(HttpResponse::Ok().finish())
        }
    })
    .expect_err("Expected error because the result arguments aren't all types.");
}

/// Receiver Args Allowed In [`proof_route`]
///
/// This tests whether receiver args don't return an error
/// in a [`proof_route`] handler.
///
/// XXX: This test is probably redundant.
///
/// [`proof_route`]: crate::proof_route
#[test]
pub fn proof_route_receiver_args() {
    parse2::<ProofRouteBody>(quote! {
        async fn x(self) -> Result<_, Error> {
            Ok(HttpResponse::Ok().finish())
        }
    })
    .expect("Expected success with receiver args.");
}

/// Error Override Invalid Input Test
///
/// This tests whether invalid input in `error_override`
/// in a [`proof_route`] handler triggers an errror.
///
/// [`proof_route`]: crate::proof_route
#[test]
pub fn proof_route_matching_arg_override() {
    parse2::<ProofRouteBody>(quote! {
        async fn x(#[error_override(fn foo() {})] a: ()) -> Result<_, Error> {
            Ok(HttpResponse::Ok().finish())
        }
    })
    .expect_err("Expected error because expected the error override to be an expr.");
}
