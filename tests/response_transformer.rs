use actix_failwrap_proc::{proof_route, ErrorResponse};
use actix_web::{HttpResponse, HttpResponseBuilder};
use thiserror::Error;

mod common;

fn error_to_header(mut code: HttpResponseBuilder, format: String) -> HttpResponse {
    code
        .insert_header(("Error", format))
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
