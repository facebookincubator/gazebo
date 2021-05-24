/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

//! Cast between types with no conversion.
//!
//! Most of these operations are inherently unsafe, but provided as strongly-typed wrappers
//! to reduce the chance of typos ending up with even more unsafe functions. If you use the
//! result in incorrect ways, it will cause undefined behaviour.

// These are inherently unsafe in fairly obvious ways. Safety is left entirely to the user.
// So doc's wouldn't really help.
#![allow(clippy::missing_safety_doc)]

pub fn ptr_to_usize<T: ?Sized>(x: &T) -> usize {
    x as *const T as *const () as usize
}

/// Undefined behaviour if the argument is zero, or does not satisfy the alignment
/// of type `T`.
pub unsafe fn usize_to_ptr<'a, T>(x: usize) -> &'a T {
    &*(x as *const T)
}

/// Undefined behaviour if the argument does not satisfy the alignment of type `To`.
pub unsafe fn ptr<From, To>(x: &From) -> &To {
    &*(x as *const From as *const To)
}

/// Undefined behaviour if the argument does not satisfy the alignment of type `To`.
pub unsafe fn ptr_mut<From, To>(x: &mut From) -> &mut To {
    &mut *(x as *mut From as *mut To)
}

pub unsafe fn ptr_lifetime<'a, 'b, T: ?Sized>(x: &'a T) -> &'b T {
    &*(x as *const T)
}

#[macro_export]
/// `transmute!(from-type, to-type, value)` will do a [`transmute`](std::mem::transmute),
/// but the original and result types must be specified.
macro_rules! transmute {
    ($from:ty, $to:ty, $e:expr) => {
        std::mem::transmute::<$from, $to>($e)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_transmute() {
        #[allow(clippy::useless_transmute)]
        unsafe fn downcast_string<'a>(x: &'a str) -> &'static str {
            transmute!(&'a str, &'static str, x)
        }
        assert_eq!(unsafe { downcast_string("test") }, "test");
    }
}
