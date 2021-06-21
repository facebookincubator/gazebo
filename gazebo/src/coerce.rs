/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

//! A trait to represent zero-cost conversions.

// TODO(ndmitchell): We could derive instances, similarly to `ref-cast`.
// Leave that as future work if it turns out to be a useful idea.

use crate::cast::{self, transmute_unchecked};

/// A marker trait such that the existence of `From: Coerce<To>` implies
/// that `From` can be treat as `To` without any data manipulation.
/// Particularly useful for containers, e.g. `Vec<From>` can be treated as
/// `Vec<To>` in _O(1)_. If such an instance is available,
/// you can use [`coerce`] and [`coerce_ref`] to perform the conversion.
///
/// Importantly, you must make sure Rust does not change the type representation
/// between the different types (typically using a `repr` directive),
/// and it must be safe for the `From` to be treated as `To`, namely same (or less restrictive) alignment,
/// no additional invariants, value can be dropped as `To`.
///
/// One use of `Coerce` is around newtype wrappers:
///
/// ```
/// use gazebo::coerce::{Coerce, coerce, coerce_ref};
/// #[repr(transparent)]
/// #[derive(Debug)]
/// struct Wrapper(String);
/// unsafe impl Coerce<String> for Wrapper {}
///
/// let value = vec![Wrapper("hello".to_owned()), Wrapper("world".to_owned())];
/// assert_eq!(
///     coerce_ref::<_, Vec<String>>(&value).join(" "),
///     "hello world"
/// );
/// let mut value = coerce::<_, Vec<String>>(value);
/// assert_eq!(value.pop(), Some("world".to_owned()));
/// ```
///
/// Another involves containers:
///
/// ```
/// use gazebo::coerce::{Coerce, coerce_ref};
/// # #[repr(transparent)]
/// # struct Wrapper(String);
/// # unsafe impl Coerce<String> for Wrapper {}
/// #[repr(C)]
/// struct Container<T>(i32, T);
/// unsafe impl<From, To> Coerce<Container<To>> for Container<From> where From: Coerce<To> {}
///
/// let value = Container(20, Wrapper("twenty".to_owned()));
/// assert_eq!(
///     coerce_ref::<_, Container<String>>(&value).1,
///     "twenty"
/// );
/// ```
///
/// If you only need [`coerce_ref`] on newtypes, then the [`ref-cast` crate](https://crates.io/crates/ref-cast)
/// provides that, along with automatic derivations (no `unsafe` required).
pub unsafe trait Coerce<To> {}

unsafe impl<From, To> Coerce<Vec<To>> for Vec<From> where From: Coerce<To> {}
unsafe impl<From, To> Coerce<Box<To>> for Box<From> where From: Coerce<To> {}

/// Safely convert between types which have a `Coerce` relationship.
/// Often the second type argument will need to be given explicitly,
/// e.g. `coerce::<_, ToType>(x)`.
pub fn coerce<From, To>(x: From) -> To
where
    From: Coerce<To>,
{
    unsafe { transmute_unchecked(x) }
}

/// Safely convert between types which have a `Coerce` relationship.
/// Often the second type argument will need to be given explicitly,
/// e.g. `coerce_ref::<_, ToType>(x)`.
pub fn coerce_ref<From, To>(x: &From) -> &To
where
    From: Coerce<To>,
{
    unsafe { cast::ptr(x) }
}
