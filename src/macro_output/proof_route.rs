use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::macro_input::proof_route::{ProofRouteBody, ProofRouteMeta};

pub fn proof_route(meta: ProofRouteMeta, body: ProofRouteBody) -> TokenStream2 {
    quote! {}
}
