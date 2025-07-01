/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is dual-licensed under either the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree or the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree. You may select, at your option, one of the
 * above-listed licenses.
 */

/// Extension traits on [`Option`](Option) where it holds any value or ref.
pub trait OptionExt {
    type Item;

    /// Like `map`, but as a `Result`
    ///
    /// ```
    /// use gazebo::prelude::*;
    ///
    /// assert_eq!(Some("foo").try_map(|x| Ok::<_, ()>(x.len())), Ok(Some(3)));
    /// assert_eq!(Some("foo").try_map(|x| Err::<(), _>(())), Err(()));
    /// ```
    fn try_map<U, E, F: FnOnce(Self::Item) -> Result<U, E>>(self, f: F) -> Result<Option<U>, E>;
}

impl<T> OptionExt for Option<T> {
    type Item = T;

    fn try_map<U, E, F: FnOnce(Self::Item) -> Result<U, E>>(self, f: F) -> Result<Option<U>, E> {
        self.map(f).transpose()
    }
}
