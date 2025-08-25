//! Response Transformer Tests
//!
//! This tests whether the transformer function works correctly,
//! by calling `GET /` on he boostrap server, the error
//! should show up on the headers.

use actix_failwrap::{ErrorResponse, proof_route};
use actix_web::{HttpResponse, HttpResponseBuilder};
use common::test_http_endpoint;
use thiserror::Error;

mod common;

fn error_to_header(mut code: HttpResponseBuilder, format: String) -> HttpResponse {
    code.insert_header(("Error", format))
        .finish()
}

#[derive(ErrorResponse, Error, Debug)]
#[transform_response(error_to_header)]
enum TestError {
    #[error("This goes on a header.")]
    TransformedError,
}

#[proof_route("GET /")]
async fn response_transformer() -> Result<HttpResponse, TestError> {
    Err(TestError::TransformedError)
}

test_http_endpoint!(
    test response_transformer as test_response_transformer
    with request {
        head: get /;
    }
    and expect response {
        head: 500;
        headers: {
            Error: "This goes on a header."
        }
    }
);
