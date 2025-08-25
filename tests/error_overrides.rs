//! Error Override tests
//!
//! This test checks whether the `#[default_status_code(..)]`
//! attribute works correctly. For the `GeneralOverride`
//! test, this should return a 400 status code and for the
//! `SpecificOverride` it should return 404.

use actix_failwrap::{ErrorResponse, proof_route};
use actix_web::HttpResponse;
use actix_web::web::Path;
use common::test_http_endpoint;
use thiserror::Error;

mod common;

#[derive(ErrorResponse, Error, Debug)]
#[default_status_code(400)]
enum TestError {
    #[error("General override.")]
    GeneralOverride,

    #[error("Specific override.")]
    #[status_code(404)]
    SpecificOverride,
}

#[proof_route("GET /{error_type}")]
async fn error_overrides(error_type: Path<String>) -> Result<HttpResponse, TestError> {
    match error_type.as_str() {
        "general" => Err(TestError::GeneralOverride),
        "specific" => Err(TestError::SpecificOverride),
        _ => unreachable!("The test shouldn't even receive any other value."),
    }
}

test_http_endpoint!(
    test error_overrides as test_error_override_general
    with request {
        head: get /general;
    }
    and expect response {
        head: 400;
        body: {
            "General override."
        }
    }
);

test_http_endpoint!(
    test error_overrides as test_error_override_specific
    with request {
        head: get /specific;
    }
    and expect response {
        head: 404;
        body: {
            "Specific override."
        }
    }
);
