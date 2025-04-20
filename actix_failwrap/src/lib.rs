use actix_web::HttpResponse;

pub use actix_failwrap_proc::ResponseError;
pub type HttpResult<E> = Result<HttpResponse, E>;

