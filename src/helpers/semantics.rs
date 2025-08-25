//! Semantics Helpers
//!
//! This module declares functions that help
//! validate semantics for parsed elements.

use syn::{GenericArgument, PathArguments, Type, TypePath};

/// **has_result_semantics()**
///
/// Parses the semantics for a `Result` like type
/// and returns a tuple with the `T` and `E` parsed types.
pub fn has_result_semantics(ty: &Type) -> Option<(&Type, &Type)> {
    let Type::Path(TypePath { path, .. }) = ty else {
        return None;
    };
    let last_seg = path
        .segments
        .last()?;

    if last_seg.ident != "Result" {
        return None;
    }

    let PathArguments::AngleBracketed(args) = &last_seg.arguments else {
        return None;
    };

    let mut args_iter = args
        .args
        .iter();

    let GenericArgument::Type(t) = args_iter.next()? else {
        return None;
    };
    let GenericArgument::Type(e) = args_iter.next()? else {
        return None;
    };

    Some((t, e))
}
