//! [`get_single_attr`] Tests
//!
//! This crate tests whether the `get_single_attr` function
//! returns the expected values.

use quote::quote;
use syn::{ItemFn, parse2};

use crate::helpers::unique_attr::get_single_attr;

/// [`get_single_attr`] Result Tests
///
/// This tests whether [`get_single_attr`]
/// returns the expected result, which is
/// true for non duplicated attributes,
/// otherwise false.
#[test]
pub fn get_single_attr_test() {
    let function = parse2::<ItemFn>(quote! {
        #[repeated]
        #[repeated]
        #[single]
        fn x() {}
    })
    .expect("ItemFn to be parsed");

    get_single_attr(
        function
            .attrs
            .clone(),
        "repeated",
    )
    .expect_err("Shouldn't be OK");
    get_single_attr(
        function
            .attrs
            .clone(),
        "single",
    )
    .expect("Should be OK");
}
