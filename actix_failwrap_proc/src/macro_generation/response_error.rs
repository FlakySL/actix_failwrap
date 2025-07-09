use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use syn::Ident;

use crate::macro_input::response_error::{MacroArgs, Status};

pub fn response_error_macro(args: MacroArgs) -> TokenStream2 {
    let enum_name = args.name();

    let transformer = args
        .macro_config()
        .and_then(|config| config.transform());

    let default_status = args
        .macro_config()
        .and_then(|config| config.default_status())
        .cloned()
        .unwrap_or(Status::Ref(Ident::new("InternalServerError", Span::call_site())));

    let branch_props = args
        .branches()
        .iter()
        .map(|(branch, status)| (
            branch.to_branch_tokens("Self", ".."),
            status.clone().unwrap_or(default_status.clone())
        ));

    let into_response_arms = branch_props
        .clone()
        .map(|(branch_tokens, status)| {
            match transformer {
                Some(transformer) => quote! {
                    #branch_tokens => #transformer(#status, self.to_string())
                },

                None => quote! {
                    #branch_tokens => #status.body(self.to_string())
                }
            }
        });

    let into_error_arms = branch_props
        .map(|(branch_tokens, status)| {
            let error_ident = format_ident!(
                "Error{}",
                status
                    .to_token_stream()
                    .to_string()
            );

            quote! {
                #branch_tokens => ::actix_web::error::#error_ident(self.to_string())
            }
        });

    quote! {
        impl ::std::convert::Into<actix_web::HttpResponse> for #enum_name {
            fn into(self) -> ::actix_web::HttpResponse {
                match self {
                    #(#into_response_arms),*
                }
            }
        }

        impl ::std::convert::Into<actix_web::Error> for #enum_name {
            fn into(self) -> ::actix_web::Error {
                match self {
                    #(#into_error_arms),*
                }
            }
        }
    }
}
