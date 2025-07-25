use actix_failwrap_proc::{proof_route, ErrorResponse};
use actix_web::{web::Path, HttpResponse};
use thiserror::Error;

mod common;

#[derive(ErrorResponse, Error, Debug)]
enum TestError {
    #[error("This is a unit type error.")]
    Unit,

    #[error("This is a tuple type error containing ({0:?}).")]
    Tuple(i32),

    #[error(
        "This is a struct type error representing a person with name {name} and age {age}.",
    )]
    Object {
        name: String,
        age: i32
    }
}

#[proof_route("GET /{error_type}")]
async fn variant_types(error_type: Path<String>) -> Result<HttpResponse, TestError> {
    match error_type.as_str() {
        "unit" => Err(TestError::Unit),
        "tuple" => Err(TestError::Tuple(69)),
        "object" => Err(TestError::Object { name: "John".into(), age: 36 }),
        _ => unreachable!("The test shouldn't even receive any other value.")
    }
}

test_http_endpoint!(
    test variant_types as test_variant_unit
    with request {
        head: get /unit;
    }
    and expect response {
        head: 500;
        body: { "This is a unit type error." }
    }
);

test_http_endpoint!(
    test variant_types as test_variant_tuple
    with request {
        head: get /tuple;
    }
    and expect response {
        head: 500;
        body: { "This is a tuple type error containing (69)." }
    }
);

test_http_endpoint!(
    test variant_types as test_variant_object
    with request {
        head: get /object;
    }
    and expect response {
        head: 500;
        body: { "This is a struct type error representing a person with name John and age 36." }
    }
);
