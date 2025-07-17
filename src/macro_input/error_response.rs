use quote::format_ident;
use syn::parse::{Parse, ParseStream};
use syn::{
    Error as SynError,
    Ident,
    ItemEnum,
    LitInt,
    Result as SynResult,
    Variant as EnumVariant,
};

use crate::helpers::status_codes::{
    allowed_status_pairs,
    closest_status,
    code_to_status,
    is_status_supported,
};
use crate::helpers::unique_attr::get_single_attr;

pub struct ErrorResponse {
    enum_name: Ident,
    default_status_code: Ident,        // by default 500. Dynamic
    transform_response: Option<Ident>, // an onscope reference Fn(HttpStatusCode, &str)
    variants: Vec<ErrorResponseVariant>,
}

pub struct ErrorResponseVariant {
    status_code: Option<Ident>,
    variant: EnumVariant,
}

pub struct StatusCode(Ident);

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
    pub const fn transform_response(&self) -> Option<&Ident> {
        self.transform_response
            .as_ref()
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

        let default_status_code = get_single_attr(
            input
                .attrs
                .clone(),
            "default_status_code",
        )?
        .map(|attr| attr.parse_args::<StatusCode>())
        .transpose()?
        .map(|code| code.into_inner())
        .unwrap_or(format_ident!("InternalServerError"));

        let transform_response = get_single_attr(input.attrs, "transform_response")?
            .map(|attr| attr.parse_args::<Ident>())
            .transpose()?;

        let variants = input
            .variants
            .into_iter()
            .map(|variant| {
                Ok(ErrorResponseVariant {
                    status_code: get_single_attr(
                        variant
                            .attrs
                            .clone(),
                        "status_code",
                    )?
                    .map(|attr| attr.parse_args::<StatusCode>())
                    .transpose()?
                    .map(|code| code.into_inner()),
                    variant,
                })
            })
            .collect::<SynResult<Vec<_>>>()?;

        Ok(Self {
            enum_name,
            default_status_code,
            transform_response,
            variants,
        })
    }
}

impl ErrorResponseVariant {
    #[inline(always)]
    pub const fn status_code(&self) -> Option<&Ident> {
        self.status_code
            .as_ref()
    }

    #[inline(always)]
    pub fn variant(&self) -> &EnumVariant {
        &self.variant
    }
}

impl StatusCode {
    pub fn into_inner(self) -> Ident {
        self.0
    }
}

impl Parse for StatusCode {
    fn parse(input: ParseStream) -> SynResult<Self> {
        if let Ok(integer) = input.parse::<LitInt>() {
            let parsed = integer
                .base10_parse::<usize>()
                .map_err(|error| {
                    SynError::new(error.span(), "Expected a usize value for number variant.")
                })?;

            let status = code_to_status(parsed).ok_or_else(|| {
                SynError::new_spanned(
                    integer,
                    format!(
                        concat!(
                            "Only HTTP error status codes are allowed, ",
                            "The allowed status codes are:\n{}"
                        ),
                        allowed_status_pairs()
                            .iter()
                            .map(|(code, status)| format!("{code} -> {status}"))
                            .collect::<String>()
                    ),
                )
            })?;

            return Ok(StatusCode(format_ident!("{status}")));
        }

        if let Ok(ident) = input.parse::<Ident>() {
            let ident_string = ident.to_string();
            return match is_status_supported(&ident_string) {
                true => Ok(StatusCode(ident)),
                false => Err(SynError::new_spanned(
                    ident,
                    format!(
                        concat!(
                            "Only HTTP error statuses are allowed. ",
                            "{} is not a valid status code{}"
                        ),
                        &ident_string,
                        match closest_status(&ident_string) {
                            Some(closest) => format!(", did you mean {closest}?"),
                            None => String::new(),
                        }
                    ),
                )),
            };
        }

        Err(SynError::new(
            input.span(),
            "Only HTTP status codes (usize) and refrences (Ident) are allowed.",
        ))
    }
}
