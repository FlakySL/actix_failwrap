use proc_macro2::TokenStream as TokenStream2;
use quote::{TokenStreamExt, format_ident, quote};

use crate::macro_input::proof_route::{ProofRouteBody, ProofRouteMeta};

pub fn proof_route_output(meta: ProofRouteMeta, body: ProofRouteBody) -> TokenStream2 {
    let http_method = format_ident!(
        "{}",
        meta.method()
            .to_lowercase()
    );
    let http_path = meta.path();

    let handler_name = body.name();
    let handler_function = body.function();

    let parameters = body
        .parameters()
        .iter()
        .enumerate()
        .fold(TokenStream2::new(), |mut acc, (idx, curr)| {
            let ident = format_ident!("_{idx}");
            let ty = curr.ty();
            let error_override = match curr.error_override() {
                Some(error) => quote! { Err(_) => return #error.into() },
                None => quote! { Err(error) => return error.into() },
            };

            acc.append_all(quote! {
                match
                <#ty as ::actix_web::FromRequest>::from_request(&__request, &mut __payload).await {
                    Ok(value) => value,
                    #error_override
                },
            });

            acc
        });

    quote! {
        #[::actix_web::#http_method(#http_path)]
        async fn #handler_name(
            __request: ::actix_web::HttpRequest,
            __payload: ::actix_web::web::Payload
        ) -> impl ::actix_web::Responder {
            #[doc(hidden)]
            #handler_function

            #[allow(unused)]
            #[doc(hidden)]
            let mut __payload = __payload.into_inner();

            match #handler_name(#parameters).await {
                ::std::result::Result::Ok(result) => result,
                ::std::result::Result::Err(error) => error.into()
            }
        }
    }
}
