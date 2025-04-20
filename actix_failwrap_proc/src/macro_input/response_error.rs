use std::collections::HashMap;

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{parse::{Parse, ParseStream}, parse2, Data, DeriveInput, Error as SynError, Fields, Ident, LitInt, MetaNameValue, Result as SynResult, Token, Variant};
use thiserror::Error;

use crate::misc::errors::IntoCompileError;

#[derive(Error, Debug)]
enum MacroError {
    #[error("Expected valid identifier")]
    InvalidIdent,

    #[error("Duplicated identifier, an identifier can't appear twice")]
    DuplicateIdent,

    #[error("Duplicated attribute, an attribute can't appear twice")]
    DuplicateAttr,

    #[error(
        "The provided key is invalid, expected any of: {expected}",
        expected = .0
            .join(", ")
    )]
    InvalidKey(Vec<String>),

    #[error(
        "The provided value is invalid, expected any of: {expected}",
        expected = .0
            .join(", ")
    )]
    InvalidValue(Vec<String>),

    #[error("This attribute can only be used in enums")]
    InvalidExpectedEnum
}

pub struct Config {
    transform: Option<Ident>,
    default_status: Option<Status>
}

pub enum Status {
    Code(u16),
    Ref(Ident)
}

pub enum ErrorVariant {
    Unit(Option<Status>),
Tuple(Option<Status>),
    Struct(Option<Status>)
}

pub struct MacroArgs {
    macro_config: Option<Config>,
    branches: HashMap<Ident, ErrorVariant>
}

impl Parse for Config {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut transform = None;
        let mut default_status = None;

        for pair in input.parse_terminated(MetaNameValue::parse, Token![,])? {
            let key = pair
                .path
                .clone();
            let key = key
                .get_ident()
                .ok_or_else(|| MacroError::InvalidIdent.to_syn_error(pair.path))?;

            let value = pair
                .value
                .to_token_stream();

            match key.to_string().as_str() {
                "transform" => {
                    if transform.is_some() {
                        return Err(MacroError::DuplicateIdent.to_syn_error(key));
                    }

                    transform = Some(
                        parse2(value.clone())?
                    );
                },

                "default_status" => {
                    if default_status.is_some() {
                        return Err(MacroError::DuplicateIdent.to_syn_error(key));
                    }

                    default_status = Some(parse2(value)?);
                },

                _ => {
                    return Err(
                        MacroError::InvalidKey(vec!["transform".into(), "default_status".into()])
                            .to_syn_error(key)
                    );
                }
            }
        }

        Ok(Self { transform, default_status })
    }
}

impl Parse for Status {
    fn parse(input: ParseStream) -> SynResult<Self> {
        if input.peek(LitInt) {
            Ok(Self::Code(input.parse::<LitInt>()?.base10_parse()?))
        } else if input.peek(Ident) {
            Ok(Self::Ref(input.parse::<Ident>()?))
        } else {
            Err(
                MacroError::InvalidValue(vec!["u16 literal".into(), "status ident".into()])
                    .to_syn_error(input.parse::<TokenStream2>()?)
            )
        }
    }
}

impl TryFrom<Variant> for ErrorVariant {
    type Error = SynError;

    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        let mut status = None;

        for attr in value.attrs {
            if attr.path().is_ident("status") {
                if status.is_some() {
                    return Err(
                        MacroError::DuplicateAttr
                            .to_syn_error(attr)
                    );
                }

                status = Some(attr.parse_args::<Status>()?);
            }
        }

        Ok(match value.fields {
            Fields::Unit => Self::Unit(status),
            Fields::Unnamed(..) => Self::Tuple(status),
            Fields::Named(..) => Self::Struct(status)
        })
    }
}

impl TryFrom<DeriveInput> for MacroArgs {
    type Error = SynError;

    fn try_from(value: DeriveInput) -> Result<Self, Self::Error> {
        let mut config = None;

        for attr in &value.attrs {
            if attr.path().is_ident("failwrap") {
                if config.is_some() {
                    return Err(
                        MacroError::DuplicateAttr
                            .to_syn_error(attr)
                    );
                }

                config = Some(attr.parse_args::<Config>()?)
            }
        }

        let Data::Enum(data) = value.data
        else {
            return Err(
                MacroError::InvalidExpectedEnum
                    .to_syn_error(value)
            );
        };

        let variants = data
            .variants
            .into_iter()
            .map(|variant| Ok((variant.ident.clone(), variant.try_into()?)))
            .collect::<Result<HashMap<Ident, ErrorVariant>, SynError>>()?;

        Ok(Self { macro_config: config, branches: variants })
    }
}
