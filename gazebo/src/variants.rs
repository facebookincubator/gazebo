/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

//! Working with the variants of an `enum`.

extern crate gazebo_derive;
pub use gazebo_derive::VariantName;

/// Trait for enums to return the name of the current variant as a `str`. Useful for
/// debugging messages.
///
/// ```
/// use gazebo::variants::VariantName;
///
/// #[derive(VariantName)]
/// enum Foo {
///     Bar,
///     Baz,
/// }
///
/// assert_eq!(Foo::Bar.variant_name(), "Bar");
/// ```
///
pub trait VariantName {
    fn variant_name(&self) -> &'static str;
}

impl<T> VariantName for Option<T> {
    fn variant_name(&self) -> &'static str {
        match self {
            Self::Some(_) => "Some",
            None => "None",
        }
    }
}

impl<T, E> VariantName for Result<T, E> {
    fn variant_name(&self) -> &'static str {
        match self {
            Self::Ok(_) => "Ok",
            Self::Err(_) => "Err",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)] // Not actually unused, this makes testing the derive macro work
    use crate as gazebo;

    #[test]
    fn derive_variant_names() {
        #[allow(unused)]
        #[derive(VariantName)]
        enum MyEnum {
            Foo,
            Bar(usize),
            Baz { field: usize },
        }

        let x = MyEnum::Foo;
        assert_eq!(x.variant_name(), "Foo");

        let x = MyEnum::Bar(1);
        assert_eq!(x.variant_name(), "Bar");

        let x = MyEnum::Baz { field: 1 };
        assert_eq!(x.variant_name(), "Baz");
    }
}
