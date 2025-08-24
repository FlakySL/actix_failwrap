use quote::quote;
use syn::parse2;

use crate::{ProofRouteBody, ProofRouteMeta};

#[test]
pub fn proof_route_meta_no_space() {
    parse2::<ProofRouteMeta>(quote! { "GET/route/test" })
        .expect_err("Expected error no space between route meta.");
}

#[test]
pub fn proof_route_meta_invalid_http_method() {
    parse2::<ProofRouteMeta>(quote! { "BADMETHOD /route/test" })
        .expect_err("Expected error no space between route meta.");
}

#[test]
pub fn proof_route_body_actix_attributes_not_allowed() {
    parse2::<ProofRouteBody>(quote! {
        #[get("/")]
        async fn x() {}
    })
    .expect_err("Expected error no actix attributes allowed.");
}

#[test]
pub fn proof_route_no_unit_type() {
    parse2::<ProofRouteBody>(quote! {
        async fn x() { () }
    })
    .expect_err("Expected error no unit type allowed as return.");
}

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

#[test]
pub fn proof_route_receiver_args() {
    parse2::<ProofRouteBody>(quote! {
        async fn x(self) -> Result<_, Error> {
            Ok(HttpResponse::Ok().finish())
        }
    })
    .expect("Expected success with receiver args.");
}

#[test]
pub fn proof_route_matching_arg_override() {
    parse2::<ProofRouteBody>(quote! {
        async fn x(#[error_override(fn foo() {})] a: ()) -> Result<_, Error> {
            Ok(HttpResponse::Ok().finish())
        }
    })
    .expect_err("Expected error because expected the error override to be an expr.");
}
