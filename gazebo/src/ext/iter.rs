/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

/// Extension traits on [`Iterator`](Iterator).
pub trait IterExt {
    type Item;

    /// Like `eq_by`, except allow the function supplied to return a `Result` type, where we `Err`
    /// on the first encounter of `Err`.
    ///
    /// ```
    /// use gazebo::prelude::*;
    ///
    /// fn double_eq_throw_on_zero(x: &usize, y: &usize) -> Result<bool, ()> {
    ///     if *x == 0 || *y == 0 {
    ///         Err(())
    ///     } else {
    ///         Ok(x * 2 == *y)
    ///     }
    /// }
    ///
    /// let x = [1, 4, 2];
    /// let y = [2, 8, 4];
    ///
    /// assert_eq!(x.iter().try_eq_by(&y, double_eq_throw_on_zero), Ok(true));
    ///
    /// let x = [1, 4, 2];
    /// let y = [2, 0, 4];
    ///
    /// assert_eq!(x.iter().try_eq_by(&y, double_eq_throw_on_zero), Err(()));
    /// ```
    fn try_eq_by<I, F, E>(self, other: I, eq: F) -> Result<bool, E>
    where
        Self: Sized,
        I: IntoIterator,
        F: FnMut(Self::Item, I::Item) -> Result<bool, E>;
}

impl<I> IterExt for I
where
    I: Iterator,
{
    type Item = I::Item;

    fn try_eq_by<O, F, E>(mut self, other: O, mut eq: F) -> Result<bool, E>
    where
        Self: Sized,
        O: IntoIterator,
        F: FnMut(Self::Item, O::Item) -> Result<bool, E>,
    {
        let mut other = other.into_iter();

        loop {
            let x = match self.next() {
                None => return Ok(other.next().is_none()),
                Some(val) => val,
            };

            let y = match other.next() {
                None => return Ok(false),
                Some(val) => val,
            };

            if !eq(x, y)? {
                return Ok(false);
            }
        }
    }
}
