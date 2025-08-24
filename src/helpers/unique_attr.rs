use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{Attribute, Error as SynError};

pub fn get_single_attr<I: IntoIterator<Item = Attribute>>(
    iter: I,
    ident: &str,
) -> Result<Option<Attribute>, SynError> {
    let attributes = iter
        .into_iter()
        .filter(|attr| {
            attr.path()
                .is_ident(ident)
        })
        .collect::<Vec<_>>();

    if let ..=1 = attributes.len() {
        Ok(attributes
            .first()
            .cloned())
    } else {
        let mut extra_attrs_iter = attributes.iter();

        let extra_attr_spans = extra_attrs_iter
            .next()
            .map_or_else(Span::call_site, |first| {
                extra_attrs_iter.fold(first.span(), |acc, curr| {
                    acc.join(curr.span())
                        .unwrap_or(acc)
                })
            });

        Err(SynError::new(extra_attr_spans, format!("{ident} attribute is allowed only once.")))
    }
}
