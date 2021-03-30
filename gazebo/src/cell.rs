/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

//! Additions to the [`Ref`](Ref) mechanism.

use std::{
    cell::Ref,
    cmp::Ordering,
    fmt::{self, Display},
    hash::{Hash, Hasher},
    ops::Deref,
};

/// A [`Ref`](Ref) that might not actually be borrowed.
/// Either a `Ptr` (a normal & style reference), or a `Ref` (like from
/// [`RefCell`](std::cell::RefCell)), but exposes all the methods available on [`Ref`](Ref).
#[derive(Debug)]
pub enum ARef<'a, T: ?Sized + 'a> {
    Ptr(&'a T),
    Ref(Ref<'a, T>),
}

impl<T: ?Sized> Deref for ARef<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            Self::Ptr(p) => p,
            Self::Ref(p) => p.deref(),
        }
    }
}

impl<'a, T: ?Sized + 'a> ARef<'a, T> {
    /// See [`Ref.clone`](Ref::clone). Not a self method since that interferes with the [`Deref`](Deref).
    #[allow(clippy::should_implement_trait)]
    pub fn clone(orig: &Self) -> Self {
        match orig {
            Self::Ptr(p) => Self::Ptr(p),
            Self::Ref(p) => Self::Ref(Ref::clone(p)),
        }
    }

    /// See [`Ref.map`](Ref::map). Not a self method since that interferes with the [`Deref`](Deref).
    pub fn map<U: ?Sized, F>(orig: ARef<'a, T>, f: F) -> ARef<'a, U>
    where
        F: FnOnce(&T) -> &U,
    {
        match orig {
            Self::Ptr(p) => ARef::Ptr(f(p)),
            Self::Ref(p) => ARef::Ref(Ref::map(p, f)),
        }
    }

    /// See [`Ref.map_split`](Ref::map_split). Not a self method since that interferes with the
    /// [`Deref`](Deref).
    pub fn map_split<U: ?Sized, V: ?Sized, F>(orig: ARef<'a, T>, f: F) -> (ARef<'a, U>, ARef<'a, V>)
    where
        F: FnOnce(&T) -> (&U, &V),
    {
        match orig {
            Self::Ptr(p) => {
                let (a, b) = f(p);
                (ARef::Ptr(a), ARef::Ptr(b))
            }
            Self::Ref(p) => {
                let (a, b) = Ref::map_split(p, f);
                (ARef::Ref(a), ARef::Ref(b))
            }
        }
    }
}

// `Ref` doesn't have many traits on it. I don't really know why - I think that's an oversight.
// & references do have many traits on them. Therefore, when being "either" we choose to do as many
// implementations as we can.

impl<T: Display + ?Sized> Display for ARef<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        ARef::deref(self).fmt(f)
    }
}

impl<T: Hash + ?Sized> Hash for ARef<'_, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ARef::deref(self).hash(state)
    }
}

impl<A: PartialEq<B> + ?Sized, B: ?Sized> PartialEq<ARef<'_, B>> for ARef<'_, A> {
    fn eq(&self, other: &ARef<'_, B>) -> bool {
        ARef::deref(self).eq(ARef::deref(other))
    }
}

impl<A: Eq + ?Sized> Eq for ARef<'_, A> {}

impl<A: PartialOrd<B> + ?Sized, B: ?Sized> PartialOrd<ARef<'_, B>> for ARef<'_, A> {
    fn partial_cmp(&self, other: &ARef<'_, B>) -> Option<Ordering> {
        ARef::deref(self).partial_cmp(ARef::deref(other))
    }
}

impl<A: Ord + ?Sized> Ord for ARef<'_, A> {
    fn cmp(&self, other: &Self) -> Ordering {
        ARef::deref(self).cmp(ARef::deref(other))
    }
}
