/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use crate::{dupe::Dupe, types::TEq};
use std::borrow::Borrow;

/// Extension traits on slices/[`Vec`](Vec).
pub trait SliceExt {
    type Item;

    /// A shorthand for `iter().map(f).collect::<Vec<_>>()`. For example:
    ///
    /// ```
    /// use gazebo::prelude::*;
    /// assert_eq!([1,2,3].map(|x| x*x), vec![1,4,9]);
    /// assert_eq!(vec![1,2,3].map(|x| x*x), vec![1,4,9]);
    /// ```
    fn map<'a, B, F>(&'a self, f: F) -> Vec<B>
    where
        F: FnMut(&'a Self::Item) -> B;

    /// A shorthand for `iter().map(f).collect::<Result<Vec<_>, _>>()`. For example:
    ///
    /// ```
    /// use gazebo::prelude::*;
    /// assert_eq!([1,2,3].try_map(|x| Ok(x*x)), Ok::<_, bool>(vec![1,4,9]));
    /// assert_eq!([1,2,-3].try_map(|x| if *x > 0 { Ok(x*x) } else { Err(false) }), Err(false));
    /// ```
    ///
    /// This function will be generalised to [`Try`](std::ops::Try) once it has been
    /// standardised.
    fn try_map<'a, B, E, F>(&'a self, f: F) -> Result<Vec<B>, E>
    where
        F: FnMut(&'a Self::Item) -> Result<B, E>;

    /// Clone each element within a vector using `clone`. For example:
    ///
    /// ```
    /// use gazebo::prelude::*;
    /// let xs: Vec<String> = vec![String::from("hello"), String::from("world")];
    /// let ys: Vec<String> = xs.clones();
    /// assert_eq!(xs, ys);
    /// ```
    fn clones(&self) -> Vec<Self::Item>
    where
        Self::Item: Clone,
    {
        self.map(Clone::clone)
    }

    /// Duplicate each element within a vector using `dupe`. For example:
    ///
    /// ```
    /// use gazebo::prelude::*;
    /// use std::sync::Arc;
    /// let xs: Vec<Arc<String>> = vec![Arc::new(String::from("hello"))];
    /// let ys: Vec<Arc<String>> = xs.clones();
    /// assert_eq!(xs, ys);
    /// ```
    fn dupes(&self) -> Vec<Self::Item>
    where
        Self::Item: Dupe,
    {
        self.map(|x| x.dupe())
    }

    /// Take ownership of each item in the vector using `to_owned`. For example:
    ///
    /// ```
    /// use gazebo::prelude::*;
    /// let xs: &[&str] = &["hello", "world"];
    /// let ys: Vec<String> = xs.owns();
    /// ```
    fn owns<'a, T, R>(&'a self) -> Vec<R>
    where
        // Important constraints are:
        // * Self::Item == &'a T
        // * Borrow<T> == R
        Self::Item: TEq<&'a T>,
        R: Borrow<T>,
        T: ToOwned<Owned = R>,
        T: 'a,
        T: ?Sized,
    {
        self.map(|x| (*x.teq_ref()).to_owned())
    }

    /// If the size of vector is 1, returns the first element
    /// Otherwise, returns None
    /// ```
    /// use gazebo::prelude::*;
    /// assert_eq!(*vec![1].as_singleton().unwrap(), 1);
    /// assert_eq!(vec!['a', 'b', 'c'].as_singleton(), None);
    fn as_singleton(&self) -> Option<&Self::Item>;
}

impl<T> SliceExt for [T] {
    type Item = T;

    fn map<'a, B, F>(&'a self, f: F) -> Vec<B>
    where
        F: FnMut(&'a Self::Item) -> B,
    {
        self.iter().map(f).collect()
    }

    fn try_map<'a, B, E, F>(&'a self, f: F) -> Result<Vec<B>, E>
    where
        F: FnMut(&'a Self::Item) -> Result<B, E>,
    {
        self.iter().map(f).collect()
    }

    fn as_singleton(&self) -> Option<&T> {
        match self {
            [x] => Some(x),
            _ => None,
        }
    }
}

/// Extension traits on [`Vec`](Vec).
pub trait VecExt {
    type Item;

    /// A shorthand for `into_iter().map(f).collect::<Vec<_>>()`. For example:
    ///
    /// ```
    /// use gazebo::prelude::*;
    /// assert_eq!(vec![1,2,3].into_map(|x| x*x), vec![1,4,9]);
    /// ```
    fn into_map<B, F>(self, f: F) -> Vec<B>
    where
        F: FnMut(Self::Item) -> B;
}

impl<T> VecExt for Vec<T> {
    type Item = T;

    fn into_map<B, F>(self, f: F) -> Vec<B>
    where
        F: FnMut(Self::Item) -> B,
    {
        self.into_iter().map(f).collect()
    }
}
