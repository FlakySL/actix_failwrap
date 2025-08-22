use actix_failwrap::{ErrorResponse, proof_route};
use actix_web::HttpResponse;
use actix_web::web::{Json, Query};
use serde::Deserialize;
use thiserror::Error;

use common::test_http_endpoint;

mod common;

#[derive(Deserialize)]
#[expect(dead_code)]
struct TestQuery {
    token: String,
}

#[derive(Deserialize)]
#[expect(dead_code)]
struct TestPerson {
    name: String,
    age: i32,
}

#[derive(ErrorResponse, Error, Debug)]
enum TestError {
    #[error("Could not parse query.")]
    CouldNotParseQuery,

    #[error("Could not parse body.")]
    CouldNotParseBody,
}

#[proof_route("GET /")]
async fn collector_overrides(
    #[error_override(CouldNotParseQuery)] _query: Query<TestQuery>,
    #[error_override(CouldNotParseBody)] mut _body: Json<TestPerson>,
) -> Result<HttpResponse, TestError> {
    Ok(HttpResponse::Ok().finish())
}

test_http_endpoint!(
    test collector_overrides as test_collector_override_non_mut
    with request {
        head: get /;
        body: {
            r#"{"name":"John","age":50}"#
        }
    }
    and expect response {
        head: 500;
        body: {
            "Could not parse query."
        }
    }
);

test_http_endpoint!(
    test collector_overrides as test_collector_override_mut
    with request {
        head: get /?token="c65acefb-d403-4094-9b9b-01d0325b66a3";
    }
    and expect response {
        head: 500;
        body: {
            "Could not parse body."
        }
    }
);
