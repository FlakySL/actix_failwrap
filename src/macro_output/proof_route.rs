use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

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
    let handler_err_type = body.return_error();

    let parameters = body.parameters();
    let parameters = parameters
        .iter()
        .enumerate()
        .fold(Vec::with_capacity(parameters.len()), |mut acc, (idx, curr)| {
            let var_name = format_ident!("__{idx}");
            let ty = curr.ty();
            let error_override = match curr.error_override() {
                Some(error) => quote! { Err(_) => {
                    // limited for cleanness.
                    let error: #handler_err_type = #handler_err_type::#error;
                    return error.into();
                } },
                None => quote! { Err(error) => return error.into() },
            };

            acc.push(quote! {
                let #var_name: #ty = match
                <#ty as ::actix_web::FromRequest>::from_request(&__request, &mut __payload).await {
                    Ok(value) => value,
                    #error_override
                };
            });

            acc
        });

    let param_references = (0..parameters.len()).map(|idx| format_ident!("__{idx}"));

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

            #(#parameters)*

            match #handler_name(#(#param_references),*).await {
                ::std::result::Result::Ok(result) => result,
                ::std::result::Result::Err(error) => error.into()
            }
        }
    }
}
