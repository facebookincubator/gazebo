/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use std::cmp::Ordering;

/// Extension traits on [`Iterator`](Iterator).
pub trait IterExt {
    type Item;

    /// Like `any`, except allow the function supplied to return a `Result` type, where we `Err`
    /// on the first encounter of `Err`.
    ///
    /// ```
    /// use gazebo::prelude::*;
    ///
    /// fn true_if_even_throw_on_zero(x: &usize) -> Result<bool, ()> {
    ///     if *x == 0 {
    ///         Err(())
    ///     } else {
    ///         Ok(x % 2 == 0)
    ///     }
    /// }
    ///
    /// let x = [1, 3, 2];
    /// assert_eq!(x.iter().try_any(true_if_even_throw_on_zero), Ok(true));
    ///
    /// let x = [1, 3, 5];
    /// assert_eq!(x.iter().try_any(true_if_even_throw_on_zero), Ok(false));
    ///
    /// let x = [1, 0, 2];
    /// assert_eq!(x.iter().try_any(true_if_even_throw_on_zero), Err(()));
    ///
    /// ```
    fn try_any<F, E>(self, any: F) -> Result<bool, E>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Result<bool, E>;

    /// Like `all`, except allow the function supplied to return a `Result` type, where we `Err`
    /// on the first encounter of `Err`.
    ///
    /// ```
    /// use gazebo::prelude::*;
    ///
    /// fn true_if_even_throw_on_zero(x: &usize) -> Result<bool, ()> {
    ///     if *x == 0 {
    ///         Err(())
    ///     } else {
    ///         Ok(x % 2 == 0)
    ///     }
    /// }
    ///
    /// let x = [2, 4, 2];
    /// assert_eq!(x.iter().try_all(true_if_even_throw_on_zero), Ok(true));
    ///
    /// let x = [1, 3, 5];
    /// assert_eq!(x.iter().try_all(true_if_even_throw_on_zero), Ok(false));
    ///
    /// let x = [2, 0, 2];
    /// assert_eq!(x.iter().try_all(true_if_even_throw_on_zero), Err(()));
    ///
    /// ```
    fn try_all<F, E>(self, any: F) -> Result<bool, E>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Result<bool, E>;

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

    /// Like `cmp_by`, except allow the function supplied to return a `Result` type, where we `Err`
    /// on the first encounter of `Err`.
    ///
    /// ```
    /// use gazebo::prelude::*;
    /// use std::cmp::Ordering;
    ///
    /// fn double_cmp_throw_on_zero(x: &usize, y: &usize) -> Result<Ordering, ()> {
    ///     if *x == 0 || *y == 0 {
    ///         Err(())
    ///     } else {
    ///         Ok((x * 2).cmp(y))
    ///     }
    /// }
    ///
    /// let x = [1, 4, 2];
    /// let y = [2, 8, 4];
    ///
    /// assert_eq!(x.iter().try_cmp_by(&y, double_cmp_throw_on_zero), Ok(Ordering::Equal));
    ///
    /// let x = [1, 2, 2];
    /// let y = [2, 8, 4];
    ///
    /// assert_eq!(x.iter().try_cmp_by(&y, double_cmp_throw_on_zero), Ok(Ordering::Less));
    ///
    /// let x = [1, 4];
    /// let y = [2, 8, 4];
    ///
    /// assert_eq!(x.iter().try_cmp_by(&y, double_cmp_throw_on_zero), Ok(Ordering::Less));
    ///
    /// let x = [1, 4, 4];
    /// let y = [2, 8, 4];
    ///
    /// assert_eq!(x.iter().try_cmp_by(&y, double_cmp_throw_on_zero), Ok(Ordering::Greater));
    ///
    /// let x = [1, 4, 2, 3];
    /// let y = [2, 8, 4];
    ///
    /// assert_eq!(x.iter().try_cmp_by(&y, double_cmp_throw_on_zero), Ok(Ordering::Greater));
    ///
    /// let x = [1, 4, 2];
    /// let y = [2, 0, 4];
    ///
    /// assert_eq!(x.iter().try_cmp_by(&y, double_cmp_throw_on_zero), Err(()));
    /// ```
    fn try_cmp_by<I, F, E>(self, other: I, cmp: F) -> Result<Ordering, E>
    where
        Self: Sized,
        I: IntoIterator,
        F: FnMut(Self::Item, I::Item) -> Result<Ordering, E>;
}

impl<I> IterExt for I
where
    I: Iterator,
{
    type Item = I::Item;

    fn try_any<F, E>(mut self, f: F) -> Result<bool, E>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Result<bool, E>,
    {
        // TODO migrate use of Result<(), Option<E>> to ControlFlow when it's no longer unstable
        fn check<T, E>(
            mut f: impl FnMut(T) -> Result<bool, E>,
        ) -> impl FnMut((), T) -> Result<(), Option<E>> {
            move |(), x| match f(x) {
                Ok(true) => Err(None),
                Ok(false) => Ok(()),
                Err(e) => Err(Some(e)),
            }
        }

        match self.try_fold((), check(f)) {
            Ok(()) => Ok(false),
            Err(None) => Ok(true),
            Err(Some(e)) => Err(e),
        }
    }

    fn try_all<F, E>(mut self, f: F) -> Result<bool, E>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Result<bool, E>,
    {
        // TODO migrate use of Result<(), Option<E>> to ControlFlow when it's no longer unstable
        fn check<T, E>(
            mut f: impl FnMut(T) -> Result<bool, E>,
        ) -> impl FnMut((), T) -> Result<(), Option<E>> {
            move |(), x| match f(x) {
                Ok(true) => Ok(()),
                Ok(false) => Err(None),
                Err(e) => Err(Some(e)),
            }
        }

        match self.try_fold((), check(f)) {
            Ok(()) => Ok(true),
            Err(None) => Ok(false),
            Err(Some(e)) => Err(e),
        }
    }

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

    fn try_cmp_by<O, F, E>(mut self, other: O, mut cmp: F) -> Result<Ordering, E>
    where
        Self: Sized,
        O: IntoIterator,
        F: FnMut(Self::Item, O::Item) -> Result<Ordering, E>,
    {
        let mut other = other.into_iter();

        loop {
            let x = match self.next() {
                None => {
                    if other.next().is_none() {
                        return Ok(Ordering::Equal);
                    } else {
                        return Ok(Ordering::Less);
                    }
                }
                Some(val) => val,
            };

            let y = match other.next() {
                None => return Ok(Ordering::Greater),
                Some(val) => val,
            };

            match cmp(x, y)? {
                Ordering::Equal => {}
                non_eq => return Ok(non_eq),
            }
        }
    }
}
