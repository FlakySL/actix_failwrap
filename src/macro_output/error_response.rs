use proc_macro2::TokenStream as TokenStream2;
use quote::{TokenStreamExt, format_ident, quote};
use syn::Fields as VariantFields;

use crate::macro_input::error_response::{ErrorResponse, ErrorResponseVariant};

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

pub fn error_response_output(input: ErrorResponse) -> TokenStream2 {
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

            http_response_variant.append_all(match input.transform_response() {
                // We or either pass the variant trough transformer_fn
                Some(transformer_fn) => quote! {{
                    let transformed: ::actix_web::HttpResponse // type checking.
                        = #transformer_fn(#http_response_tokens, self.to_string());

                    transformed
                }},

                // Or if none we directly construct it.
                None => quote! {
                    #http_response_tokens
                        .body(self.to_string())
                },
            });

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
