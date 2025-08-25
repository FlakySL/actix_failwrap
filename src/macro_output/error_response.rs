//! [`ErrorResponse`] Output Module
//!
//! Declares the macro generation functions,
//! abstracting the macro generation instead
//! of declaring it into the main module.

use proc_macro2::TokenStream as TokenStream2;
use quote::{TokenStreamExt, format_ident, quote};
use syn::Fields as VariantFields;

use crate::macro_input::error_response::{ErrorResponse, ErrorResponseVariant};

/// **variant_match_head()**
///
/// Generates an 'ignore all' head depending on the
/// provided enum variant. Returns a [`TokenStream2`]
/// with the generated head.
fn variant_match_head(variant: &ErrorResponseVariant) -> TokenStream2 {
    let variant_name = &variant
        .variant()
        .ident;
    let variant_head_type = match variant
        .variant()
        .fields
    {
        VariantFields::Named(_) => quote! { {..} },
        VariantFields::Unnamed(_) => quote! { (..) },
        VariantFields::Unit => quote! {},
    };

    quote! { Self::#variant_name #variant_head_type => }
}

/// **error_response_output()**
///
/// Generates the [`ErrorResponse`] macro output.
pub fn error_response_output(input: &ErrorResponse) -> TokenStream2 {
    let enum_name = input.enum_name();

    let (http_response_variants, error_variants) = input
        .variants()
        .iter()
        .map(|variant| {
            let status_code = variant
                .status_code()
                .unwrap_or(input.default_status_code());

            let variant_head = variant_match_head(variant);

            let mut http_response_variant = variant_head.clone();
            let http_response_tokens = quote! {
                ::actix_web::HttpResponse::#status_code()
            };

            http_response_variant.append_all(
                if let Some(transformer_fn) = input.transform_response() {
                    quote! {{
                        let transformed: ::actix_web::HttpResponse // type checking.
                            = #transformer_fn(#http_response_tokens, self.to_string());

                        transformed
                    }}
                } else {
                    quote! {
                        #http_response_tokens
                            .body(self.to_string())
                    }
                },
            );

            let mut error_variant = variant_head.clone();
            let error_variant_ident = format_ident!("Error{status_code}");
            error_variant
                .append_all(quote! { ::actix_web::error::#error_variant_ident(self.to_string()) });

            (http_response_variant, error_variant)
        })
        .collect::<(Vec<_>, Vec<_>)>();

    quote! {
        impl ::std::convert::Into<::actix_web::HttpResponse> for #enum_name {
            fn into(self) -> ::actix_web::HttpResponse {
                match self {
                    #(#http_response_variants),*
                }
            }
        }

        impl ::std::convert::Into<::actix_web::Error> for #enum_name {
            fn into(self) -> ::actix_web::Error {
                match self {
                    #(#error_variants),*
                }
            }
        }
    }
}
