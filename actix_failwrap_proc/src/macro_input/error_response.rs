use quote::format_ident;
use syn::{Ident, ItemEnum, Result as SynResult, Variant as EnumVariant};
use syn::parse::{Parse, ParseStream};

use crate::helpers::unique_attr::get_single_attr;

pub struct ErrorResponse {
    enum_name: Ident,
    default_status_code: Ident, // by default 500. Dynamic
    transform_response: Option<Ident>, // an onscope reference Fn(HttpStatusCode, &str)
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
    pub fn enum_name(&self) -> &Ident {
        &self.enum_name
    }

    #[inline(always)]
    pub fn default_status_code(&self) -> &Ident {
        &self.default_status_code
    }

    #[inline(always)]
    pub fn transform_response(&self) -> Option<&Ident> {
        self.transform_response.as_ref()
    }

    #[inline(always)]
    pub fn variants(&self) -> &[ErrorResponseVariant] {
        &self.variants
    }
}

impl Parse for ErrorResponse {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let input = input.parse::<ItemEnum>()?;

        let enum_name = input.ident;

        let default_status_code = get_single_attr(input.attrs.clone(), "default_status_code")?
            .map(|attr| attr.parse_args::<Ident>())
            .transpose()?
            .unwrap_or(format_ident!("InternalServerError"));

        let transform_response = get_single_attr(input.attrs, "transform_response")?
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

        Ok(Self { enum_name, default_status_code, transform_response, variants })
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
