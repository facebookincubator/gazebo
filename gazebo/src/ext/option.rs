/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use crate::dupe::Dupe;

/// Extension traits on [`Option`](Option).
pub trait OptionExt {
    type Item;

    /// Like `cloned`, but with a `Dupe` constraint.
    ///
    /// ```
    /// use gazebo::prelude::*;
    /// use std::rc::Rc;
    /// let rc = Rc::new("test");
    /// assert_eq!(Some(&rc).duped(), Some(rc));
    /// assert_eq!(None::<&Rc<String>>.duped(), None);
    /// ```
    fn duped(self) -> Option<Self::Item>
    where
        Self::Item: Dupe;
}

impl<'a, T> OptionExt for Option<&'a T> {
    type Item = T;

    fn duped(self) -> Option<T>
    where
        T: Dupe,
    {
        self.map(|x| x.dupe())
    }
}