use actix_failwrap_proc::{ErrorResponse, proof_route};
use actix_web::HttpResponse;
use actix_web::web::Path;
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
