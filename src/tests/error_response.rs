use quote::quote;
use syn::parse2;

use crate::ErrorResponse;

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

#[test]
pub fn parse_error_at_least_one_variant() {
    parse2::<ErrorResponse>(quote! {
        enum Error { }
    })
    .expect_err("Expected error required at least one variant.");
}
