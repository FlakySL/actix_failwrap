//! [`proof_route`] Input Module
//!
//! Declares meta structs that implement
//! parsing methods for the [`proof_route`]
//! macro.
//!
//! [`proof_route`]: crate::proof_route

use syn::parse::{Parse, ParseStream};
use syn::{
    Attribute,
    Error as SynError,
    Expr,
    FnArg,
    Ident,
    ItemFn,
    LitStr,
    Result as SynResult,
    ReturnType,
    Type,
};

use crate::helpers::semantics::has_result_semantics;

/// **`HTTP_METHODS`**
///
/// An array of HTTP method string literals.
const HTTP_METHODS: [&str; 9] =
    ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS", "CONNECT", "TRACE"];

/// **`ProofRouteMeta`**
///
/// Metadata to crate the `actix_web` route inside the
/// wrapper.
#[derive(Debug)]
pub struct ProofRouteMeta {
    method: String,
    path: String,
}

/// **`ProofRouteBody`**
///
/// Handler function metadata.
#[derive(Debug)]
pub struct ProofRouteBody {
    name: Ident,
    parameters: Vec<ProofRouteParameter>,
    return_result_semantics: (Type, Type),
    function: ItemFn,
}

/// **`ProofRouteMeta`**
///
/// [`ProofRouteBody`] function input metadata.
#[derive(Debug)]
pub struct ProofRouteParameter {
    error_override: Option<Expr>,
    ty: Type,
}

impl ProofRouteMeta {
    /// **`ProofRouteMeta.method`**
    ///
    /// The HTTP status method compatible as an
    /// `actix_web` method attribute macro.
    #[inline]
    pub fn method(&self) -> &str {
        &self.method
    }

    /// **`ProofRouteMeta.path`**
    ///
    /// The HTTP path for this handler.
    #[inline]
    pub fn path(&self) -> &str {
        &self.path
    }
}

impl Parse for ProofRouteMeta {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let lit_str = input.parse::<LitStr>()?;

        let Some((method, path)) = lit_str
            .value()
            .split_once(' ')
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
    /// **`ProofRouteBody.name`**
    ///
    /// The handler function original name.
    #[inline]
    pub fn name(&self) -> &Ident {
        &self.name
    }

    /// **`ProofRouteBody.parameters`**
    ///
    /// The handler function input parameters.
    #[inline]
    pub fn parameters(&self) -> &[ProofRouteParameter] {
        &self.parameters
    }

    /// **`ProofRouteBody.return_result_semantics`**
    ///
    /// The semantics ie. The output of [`has_result_semantics`]
    /// applied to the return type of the function.
    #[inline]
    pub fn return_result_semantics(&self) -> &(Type, Type) {
        &self.return_result_semantics
    }

    /// **`ProofRouteBody.function`**
    ///
    /// A reference to the original unparsed function
    /// from the AST.
    #[inline]
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
                        .map(Attribute::parse_args::<Expr>)
                        .transpose()
                        .map_err(|err| SynError::new(err.span(), "Expected an expression."))?,
                    ty: *parameter.ty,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let output_match_err = SynError::new_spanned(
            &function
                .sig
                .output,
            "Expected a Result<T: actix_web::Responder, E: Into<actix_web::HttpResponse>>",
        );

        let (r_t, r_e) = if let ReturnType::Type(_, ty) = &function
            .sig
            .output
        {
            has_result_semantics(ty).ok_or(output_match_err)?
        } else {
            return Err(output_match_err);
        };

        for mut input in &mut function
            .sig
            .inputs
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

        Ok(Self {
            name,
            parameters,
            return_result_semantics: (r_t.clone(), r_e.clone()),
            function,
        })
    }
}

impl ProofRouteParameter {
    /// **`ProofRouteParameter.error_override`**
    ///
    /// The error override for this collector if annotated
    /// with `#[error_override(..)]`, otherwise None.
    #[inline]
    pub const fn error_override(&self) -> Option<&Expr> {
        self.error_override
            .as_ref()
    }

    /// **`ProofRouteParameter.ty`**
    ///
    /// The collector type.
    #[inline]
    pub fn ty(&self) -> &Type {
        &self.ty
    }
}
