use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{
    Error as SynError,
    Expr,
    FnArg,
    GenericArgument,
    Ident,
    ItemFn,
    LitStr,
    PathArguments,
    Result as SynResult,
    ReturnType,
    Type,
};

const HTTP_METHODS: [&str; 9] =
    ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS", "CONNECT", "TRACE"];

pub struct ProofRouteMeta {
    method: String,
    path: String,
}

pub struct ProofRouteBody {
    name: Ident,
    parameters: Vec<ProofRouteParameter>,
    return_error: Type,
    function: ItemFn,
}

pub struct ProofRouteParameter {
    error_override: Option<Expr>,
    ty: Type,
}

impl ProofRouteMeta {
    #[inline(always)]
    pub fn method(&self) -> &str {
        &self.method
    }

    #[inline(always)]
    pub fn path(&self) -> &str {
        &self.path
    }
}

impl Parse for ProofRouteMeta {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let lit_str = input.parse::<LitStr>()?;

        let Some((method, path)) = lit_str
            .value()
            .split_once(" ")
            .map(|(m, p)| (m.to_string(), p.to_string()))
        else {
            return Err(SynError::new_spanned(
                lit_str,
                concat!(
                    "Expected a space in between the method and the path,",
                    " example: \"<method> <path>\"."
                ),
            ));
        };

        if !HTTP_METHODS
            .iter()
            .any(|available| available.eq_ignore_ascii_case(&method))
        {
            return Err(SynError::new_spanned(
                &method,
                format!(
                    "{method} is not a valid HTTP method, expected any of {}",
                    HTTP_METHODS.join(", ")
                ),
            ));
        }

        Ok(Self { method, path })
    }
}

impl ProofRouteBody {
    #[inline(always)]
    pub fn name(&self) -> &Ident {
        &self.name
    }

    #[inline(always)]
    pub fn parameters(&self) -> &[ProofRouteParameter] {
        &self.parameters
    }

    #[inline(always)]
    pub fn return_error(&self) -> &Type {
        &self.return_error
    }

    #[inline(always)]
    pub fn function(&self) -> &ItemFn {
        &self.function
    }
}

impl Parse for ProofRouteBody {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut function = input.parse::<ItemFn>()?;

        let name = function
            .sig
            .ident
            .clone();

        if let Some(actix_attribute) = function
            .attrs
            .iter()
            .find(|attribute| {
                HTTP_METHODS
                    .iter()
                    .any(|method| {
                        attribute
                            .path()
                            .is_ident(&method.to_lowercase())
                    })
            })
        {
            return Err(SynError::new_spanned(
                actix_attribute,
                "Base actix route attributes are not allowed while using proof_route.",
            ));
        }

        let return_error = match &function
            .sig
            .output
        {
            // If the return type is (), disallow with a custom message.
            ReturnType::Default => {
                return Err(SynError::new_spanned(
                    &name,
                    "A proof handler cannot return unit type (), it must return a \
                     Result<HttpResponse, ?>.",
                ));
            },

            // If it's a type check the type
            ReturnType::Type(_, ty) => {
                // If the type is not a path it may possibly not be an error
                // so disallow.
                let Type::Path(ty) = ty.as_ref() else {
                    return Err(SynError::new_spanned(
                        ty,
                        concat!(
                            "A proof handler cannot return pointers, references, dynamic types... ",
                            "It must always return a Result<HttpResponse, ?>."
                        ),
                    ));
                };

                // If the last segment is not Result disallow,
                // this can be spoofed with a type alias or another unrelated type,
                // but will probably fail.
                let Some(last_return_segment) = ty
                    .path
                    .segments
                    .last()
                    .take_if(|segment| segment.ident == "Result")
                else {
                    return Err(SynError::new_spanned(
                        ty,
                        "A proof handler can only return a Result<HttpResponse, ?>.",
                    ));
                };

                match &last_return_segment.arguments {
                    // The type arguments must be generics <>.
                    PathArguments::AngleBracketed(arguments) => {
                        // There must be two of them, not 0, 1 or 3
                        if arguments
                            .args
                            .len()
                            != 2
                        {
                            return Err(SynError::new_spanned(
                                ty,
                                "The provided Result type doesn't require 2 generic arguments.",
                            ));
                        }

                        // Check the argument types
                        match (&arguments.args[0], &arguments.args[1]) {
                            // If the arguments are <_, Error> its "inferred".
                            (GenericArgument::Type(Type::Infer(_)), GenericArgument::Type(err)) => {
                                err.clone()
                            },

                            // If the arguments are <HttpResponse, Error> that's the target type.
                            (
                                GenericArgument::Type(Type::Path(res)),
                                GenericArgument::Type(err),
                            ) => {
                                if !res
                                    .path
                                    .is_ident("HttpResponse")
                                {
                                    return Err(SynError::new_spanned(
                                        res,
                                        concat!(
                                            "The provided route handler returns a Result. ",
                                            "But this result's ok value isn't an HttpResponse"
                                        ),
                                    ));
                                }

                                // We don't use the "HttpResponse" tokens, we just asume them.
                                err.clone()
                            },

                            // Any other thing, throw out.
                            _ => {
                                return Err(SynError::new_spanned(
                                    ty,
                                    concat!(
                                        "The provided route handler returns a Result, ",
                                        "but the generic constraints aren't all types."
                                    ),
                                ));
                            },
                        }
                    },

                    PathArguments::Parenthesized(_) => {
                        return Err(SynError::new_spanned(
                            ty,
                            concat!(
                                "A proof handler can only return a Result<HttpResponse, ?>. ",
                                "The actual return type seems to be function-like."
                            ),
                        ));
                    },

                    PathArguments::None => {
                        return Err(SynError::new_spanned(
                            ty,
                            concat!(
                                "A proof handler must return a Result<HttpResponse, ?>, while the ",
                                "actual return type has Result as identifier, it doesn't contain ",
                                "generic types... Are you sure you are using \
                                 ::std::result::Result?"
                            ),
                        ));
                    },
                }
            },
        };

        let parameters = function
            .sig
            .inputs
            .iter_mut()
            .filter_map(|parameter| match parameter {
                FnArg::Receiver(_) => None,
                FnArg::Typed(typed) => Some(typed.clone()),
            })
            .map(|parameter| {
                Ok::<_, SynError>(ProofRouteParameter {
                    error_override: parameter
                        .attrs
                        .iter()
                        .find(|attribute| {
                            attribute
                                .path()
                                .is_ident("error_override")
                        })
                        .map(|attribute| attribute.parse_args::<Expr>())
                        .transpose()
                        .map_err(|err| {
                            SynError::new(
                                err.span(),
                                format!(
                                    "Expected a {} variant.",
                                    return_error.to_token_stream()
                                ),
                            )
                        })?,
                    ty: *parameter.ty,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        for mut input in function
            .sig
            .inputs
            .iter_mut()
        {
            if let FnArg::Typed(typed) = &mut input {
                typed
                    .attrs
                    .retain(|attr| {
                        !attr
                            .path()
                            .is_ident("error_override")
                    });
            }
        }

        Ok(Self { name, parameters, return_error, function })
    }
}

impl ProofRouteParameter {
    #[inline(always)]
    pub const fn error_override(&self) -> Option<&Expr> {
        self.error_override
            .as_ref()
    }

    #[inline(always)]
    pub fn ty(&self) -> &Type {
        &self.ty
    }
}
