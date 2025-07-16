use proc_macro2::Span;
use syn::{Attribute, Error as SynError};
use syn::spanned::Spanned;


pub fn get_single_attr<I: IntoIterator<Item = Attribute>>(iter: I, ident: &str) -> Result<Option<Attribute>, SynError> {
    let attributes = iter
    .into_iter()
    .filter(|attr| attr.path().is_ident(ident))
    .collect::<Vec<_>>();

    match attributes.len() {
        ..=1 => Ok(attributes
            .first()
            .cloned()
        ),

        2.. => {
            let mut extra_attrs_iter = attributes
                .iter();

            let extra_attr_spans = extra_attrs_iter
                .next()
                .map(|first| extra_attrs_iter
                    .fold(first.span(), |acc, curr| acc.join(curr.span()).unwrap_or(acc))
                )
                .unwrap_or_else(|| Span::call_site());

            return Err(SynError::new(
                extra_attr_spans,
                format!("{ident} attribute is allowed only once.")
            ));
        },
    }
}
