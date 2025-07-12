use quote::format_ident;
use syn::{Ident, ItemEnum, Result as SynResult, Variant as EnumVariant};
use syn::parse::{Parse, ParseStream};

use crate::helpers::unique_attr::get_single_attr;

pub struct ErrorResponse {
    default_status_code: Ident, // by default 500. Dynamic
    transformer: Option<Ident>, // an onscope reference Fn(HttpStatusCode, &str)
    variants: Vec<ErrorResponseVariant>
}

pub struct ErrorResponseVariant {
    status_code: Option<Ident>,
    variant: EnumVariant
}

pub enum StatusCode {
    Number(i8),
    Identifier(Ident)
}

impl ErrorResponse {
    #[inline(always)]
    pub fn default_status_code(&self) -> &Ident {
        &self.default_status_code
    }

    #[inline(always)]
    pub fn transformer(&self) -> Option<&Ident> {
        self.transformer.as_ref()
    }

    #[inline(always)]
    pub fn variants(&self) -> &[ErrorResponseVariant] {
        &self.variants
    }
}

impl Parse for ErrorResponse {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let input = input.parse::<ItemEnum>()?;

        let default_status_code = get_single_attr(input.attrs.clone(), "default_status_code")?
            .map(|attr| attr.parse_args::<Ident>())
            .transpose()?
            .unwrap_or(format_ident!("InternalServerError"));

        let transformer = get_single_attr(input.attrs, "transformer")?
            .map(|attr| attr.parse_args::<Ident>())
            .transpose()?;

        let variants = input.variants
            .into_iter()
            .map(|variant| Ok(ErrorResponseVariant {
                status_code: get_single_attr(variant.attrs.clone(), "status_code")?
                    .map(|attr| attr.parse_args::<Ident>())
                    .transpose()?,
                variant
            }))
            .collect::<SynResult<Vec<_>>>()?;

        Ok(Self { default_status_code, transformer, variants })
    }
}

impl ErrorResponseVariant {
    #[inline(always)]
    pub fn status_code(&self) -> Option<&Ident> {
        self.status_code.as_ref()
    }

    #[inline(always)]
    pub fn variant(&self) -> &EnumVariant {
        &self.variant
    }
}
