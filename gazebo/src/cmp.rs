/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

//! Traits to help implementing dynamic comparisons.

pub use gazebo_derive::{MaybeEq, MaybeEq_Never};

use std::any::Any;

/// A comparable "token" that can be returned to wrap a reference to an [`Any`
/// type](Any) for [`PartialEq`](PartialEq).
///
/// This lets dyn traits be comparable by having all implementations return some
/// "token" that can be considered [`PartialEq`](PartialEq).
pub struct PartialEqAny<'a> {
    cmp: Box<dyn Fn(&PartialEqAny<'a>) -> bool + 'a>,
    val: &'a (dyn Any + 'static),
}

impl<'a> PartialEqAny<'a> {
    pub fn new<A: PartialEq + 'static>(a: &'a A) -> Self {
        Self {
            cmp: Box::new(move |other| Some(a) == other.get_as::<A>()),
            val: a,
        }
    }

    /// gets an instance that always compares to false
    pub fn always_false() -> Self {
        struct AlwaysFalse;

        impl PartialEq for AlwaysFalse {
            fn eq(&self, _other: &Self) -> bool {
                false
            }
        }

        PartialEqAny::new(&AlwaysFalse)
    }

    fn get_as<T: 'static>(&self) -> Option<&'a T> {
        self.val.downcast_ref::<T>()
    }
}

impl<'a> PartialEq for PartialEqAny<'a> {
    fn eq(&self, other: &PartialEqAny<'a>) -> bool {
        (self.cmp)(other)
    }
}

/// Marker to make any type "maybe" comparable by equality.
/// Types that are comparable should override the default `maybe_eq_any'` implementation to return
/// a [`PartialEqAny`](PartialEqAny) of something comparable (e.g. `self`).
///
/// [`MaybeEq`](MaybeEq) types can be derived on types using the derive macros [`MaybeEq`](MaybeEq)
/// and [`MaybeEq_Never`](MaybeEq_Never) exported via this module.
/// `#[derive(MaybeEq)]` derives a type for which it is always comparable. This requires the type
/// itself to be [`PartialEq`](PartialEq).
/// `#[derive(MaybeEq_Never)]` derives a type that is never comparable, such that
/// [`maybe_eq`](maybe_eq) always evaluates to [`None`](None).
pub trait MaybeEq {
    /// indicates whether the type is comparable. Implementors of this trait will override this
    fn is_comparable() -> bool {
        false
    }

    /// gets the actual comparable token for this type. This function is never called if
    /// [`is_comparable`](MaybeEq::is_comparable) returns `false`.
    fn get_comparable_any(_this: &Self) -> PartialEqAny {
        assert!(
            Self::is_comparable(),
            "you should only call this if is_comparable is true"
        );
        unreachable!()
    }
}

/// Compares a type `T` that is maybe comparable
///
/// ```
/// use gazebo::cmp::{MaybeEq, MaybeEq_Never, maybe_eq, PartialEqAny};
///
/// #[derive(MaybeEq_Never)]
/// struct NotComparable;
///
/// assert_eq!(maybe_eq(&NotComparable, &NotComparable), None);
///
/// #[derive(PartialEq, MaybeEq)]
/// struct Comparable(usize);
///
/// assert_eq!(maybe_eq(&Comparable(1), &Comparable(1)), Some(true));
/// assert_eq!(maybe_eq(&Comparable(1), &Comparable(2)), Some(false));
///
/// ```
pub fn maybe_eq<T>(x: &T, y: &T) -> Option<bool>
where
    T: MaybeEq,
{
    if T::is_comparable() {
        Some(T::get_comparable_any(x) == T::get_comparable_any(y))
    } else {
        None
    }
}

/// Performs a chain of comparison operation expressions yielding `std::cmp::Ordering`, supporting
/// early exit upon hitting the first expressions that doesn't yield `std::cmp::Ordering::Equal`
/// and returning the result of that. This is useful for easily writing a sequence of expressions
/// necessary to yield a comparison result.
/// The macro is expanded inplace, so any expressions dealing with `Result` types are allowed
/// provided that the larger scope allows returning result.
///
/// ```
/// use std::cmp::Ordering;
/// use gazebo::cmp_chain;
///
/// assert_eq!(
///     cmp_chain! {
///         1.cmp(&1),
///         Ok::<_, ()>(2.cmp(&2))?,
///         3.cmp(&4),
///         panic!("won't reach this"),
///     },
///     Ordering::Less,
/// );
///
/// # Ok::<_, ()>(())
/// ```
#[macro_export]
macro_rules! cmp_chain {
    ($e:expr) => {
        $e
    };
    ($e:expr, $($x:expr),+ $(,)?) => {
        match $e {
            std::cmp::Ordering::Equal => {
                cmp_chain!($($x),+)
            },
            c => {
                c
            }
        }
    };
}

/// Performs a chain of equals operation expressions yielding `bool`, supporting
/// early exit upon hitting the first expressions that returns `false` and returning `false`.
/// This is useful for easily writing a sequence of equals expressions necessary to yield a `bool`
/// The macro is expanded inplace, so any expressions dealing with `Result` types are allowed
/// provided that the larger scope allows returning result.
///
/// ```
/// use gazebo::eq_chain;
///
/// assert_eq!(
///     eq_chain! {
///         1 == 1,
///         Ok::<_, ()>(2 == 2)?,
///         3 == 4,
///         panic!("won't reach this"),
///     },
///     false,
/// );
///
/// # Ok::<_, ()>(())
/// ```
#[macro_export]
macro_rules! eq_chain {
    ($e:expr) => {
        $e
    };
    ($e:expr, $($x:expr),+ $(,)?) => {
        if $e {
            eq_chain!($($x),+)
        } else {
            false
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::cmp::PartialEqAny;
    use std::cmp::Ordering;

    struct Wrap<T>(T);

    impl<T: PartialEq + 'static> Wrap<T> {
        fn token(&self) -> PartialEqAny {
            PartialEqAny::new(&self.0)
        }
    }

    #[test]
    fn test_cmp_any() {
        let w1 = Wrap(1);
        let w2 = Wrap(1);
        let w3 = Wrap(2);

        assert_eq!(w1.token() == w2.token(), true);
        assert_eq!(w1.token() == w3.token(), false);

        let w4 = Wrap("foo");
        let w5 = Wrap("foo");
        let w6 = Wrap("bar");

        assert_eq!(w4.token() == w5.token(), true);
        assert_eq!(w4.token() == w6.token(), false);

        assert_eq!(w1.token() == w6.token(), false);
    }

    #[test]
    #[allow(clippy::eq_op)]
    fn always_false_cmp() {
        let w = Wrap(1);
        let f = PartialEqAny::always_false();

        assert_eq!(f == f, false);
        assert_eq!(f == w.token(), false);
    }

    #[test]
    fn cmp_eq_chain() {
        struct FakeComparable(
            Box<dyn Fn() -> Ordering>,
            Box<dyn Fn() -> Ordering>,
            Box<dyn Fn() -> Ordering>,
        );
        impl PartialEq for FakeComparable {
            fn eq(&self, _other: &Self) -> bool {
                eq_chain!(
                    (self.0)() == Ordering::Equal,
                    (self.1)() == Ordering::Equal,
                    (self.2)() == Ordering::Equal,
                )
            }
        }
        impl Eq for FakeComparable {}
        impl PartialOrd for FakeComparable {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for FakeComparable {
            fn cmp(&self, _other: &Self) -> Ordering {
                cmp_chain! {
                    (self.0)(),
                    (self.1)(),
                    (self.2)(),
                }
            }
        }

        let fake = FakeComparable(
            Box::new(|| Ordering::Less),
            Box::new(|| unreachable!("should return less")),
            Box::new(|| unreachable!("should return less")),
        );
        assert_eq!(fake.cmp(&fake), Ordering::Less);
        assert_eq!(fake.eq(&fake), false);

        let fake = FakeComparable(
            Box::new(|| Ordering::Greater),
            Box::new(|| unreachable!("should return less")),
            Box::new(|| unreachable!("should return less")),
        );
        assert_eq!(fake.cmp(&fake), Ordering::Greater);
        assert_eq!(fake.eq(&fake), false);

        let fake = FakeComparable(
            Box::new(|| Ordering::Equal),
            Box::new(|| Ordering::Less),
            Box::new(|| unreachable!("should return less")),
        );
        assert_eq!(fake.cmp(&fake), Ordering::Less);
        assert_eq!(fake.eq(&fake), false);

        let fake = FakeComparable(
            Box::new(|| Ordering::Equal),
            Box::new(|| Ordering::Equal),
            Box::new(|| Ordering::Greater),
        );
        assert_eq!(fake.cmp(&fake), Ordering::Greater);
        assert_eq!(fake.eq(&fake), false);

        let fake = FakeComparable(
            Box::new(|| Ordering::Equal),
            Box::new(|| Ordering::Equal),
            Box::new(|| Ordering::Equal),
        );
        assert_eq!(fake.cmp(&fake), Ordering::Equal);
        assert_eq!(fake.eq(&fake), true);
    }
}

// Implementations of [`MaybeEq`](MaybeEq) for primitive types
mod impls {
    use crate::{
        cell::ARef,
        cmp::{MaybeEq, PartialEqAny},
    };
    use std::{boxed::Box, sync::Arc};

    macro_rules! wrapped_impl {
        ($($t:ty)*) => ($(
            impl<T> MaybeEq for $t where T : MaybeEq + ?Sized {
                fn is_comparable() -> bool {
                    T::is_comparable()
                }

                fn get_comparable_any(this: &Self) -> PartialEqAny {
                    T::get_comparable_any(&**this)
                }
            }
        )*)
    }

    wrapped_impl!(Arc<T> Box<T> ARef<'_, T>);

    macro_rules! eq_impl {
        ($($t:ty)*) => ($(
            impl MaybeEq for $t {
                 fn is_comparable() -> bool {
                     true
                 }

                 fn get_comparable_any(this: &Self) -> PartialEqAny {
                     PartialEqAny::new(this)
                 }
            }
        )*)
    }

    eq_impl!(() bool u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize String);

    /// [`Result`](Result) types are [`MaybeEq`](MaybeEq) if both the result and the error types
    /// are [`MaybeEq`](MaybeEq)
    impl<T, E> MaybeEq for Result<T, E>
    where
        T: MaybeEq,
        E: MaybeEq,
    {
        fn is_comparable() -> bool {
            T::is_comparable() && E::is_comparable()
        }

        fn get_comparable_any(this: &Self) -> PartialEqAny {
            match this {
                Ok(t) => T::get_comparable_any(t),
                Err(e) => E::get_comparable_any(e),
            }
        }
    }

    impl<T> MaybeEq for Vec<T>
    where
        T: MaybeEq + 'static,
    {
        fn is_comparable() -> bool {
            T::is_comparable()
        }

        fn get_comparable_any(this: &Self) -> PartialEqAny {
            /// this provides an allocation free "view" over the vector that provides the equals
            /// functionality
            #[repr(transparent)]
            struct View<T>(Vec<T>);

            impl<T> PartialEq for View<T>
            where
                T: MaybeEq,
            {
                fn eq(&self, other: &Self) -> bool {
                    if self.0.len() != other.0.len() {
                        return false;
                    }

                    let this = self.0.iter().map(MaybeEq::get_comparable_any);
                    let other = other.0.iter().map(MaybeEq::get_comparable_any);

                    this.eq(other)
                }
            }

            PartialEqAny::new(unsafe {
                // we do a ref cast from the vector into the view
                // Ideally, we would use the ref_cast crate, but we do this ourselves to avoid
                // taking on an extra dependency.
                &*(this as *const Vec<T> as *const View<T>)
            })
        }
    }

    impl<T> MaybeEq for Option<T>
    where
        T: MaybeEq + 'static,
    {
        fn is_comparable() -> bool {
            T::is_comparable()
        }

        fn get_comparable_any(this: &Self) -> PartialEqAny {
            /// this provides an allocation free "view" over the option that provides the equals
            /// functionality
            #[repr(transparent)]
            struct View<T>(Option<T>);

            impl<T> PartialEq for View<T>
            where
                T: MaybeEq + 'static,
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0.as_ref().map(T::get_comparable_any)
                        == other.0.as_ref().map(T::get_comparable_any)
                }
            }

            PartialEqAny::new(unsafe {
                // we do a ref cast from the vector into the view
                // Ideally, we would use the ref_cast crate, but we do this ourselves to avoid
                // taking on an extra dependency.
                &*(this as *const Option<T> as *const View<T>)
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::cmp::{maybe_eq, MaybeEq, MaybeEq_Never};

        #[allow(unused_imports)]
        // Not actually unused, this makes testing the derive macro work
        use crate as gazebo;

        #[derive(PartialEq, MaybeEq)]
        struct MaybeEqFoo(i32);

        #[derive(PartialEq, MaybeEq)]
        struct MaybeEqErr;

        #[derive(MaybeEq_Never)]
        struct NotMaybeEqFoo;

        #[test]
        fn result_maybe_eq() {
            assert_eq!(
                maybe_eq(
                    &Ok::<_, MaybeEqErr>(MaybeEqFoo(1)),
                    &Ok::<_, MaybeEqErr>(MaybeEqFoo(1)),
                ),
                Some(true)
            );
            assert_eq!(
                maybe_eq(
                    &Ok::<_, MaybeEqErr>(MaybeEqFoo(1)),
                    &Ok::<_, MaybeEqErr>(MaybeEqFoo(2)),
                ),
                Some(false)
            );
            assert_eq!(
                maybe_eq(
                    &Ok::<_, MaybeEqErr>(MaybeEqFoo(1)),
                    &Err::<MaybeEqFoo, _>(MaybeEqErr),
                ),
                Some(false)
            );
            assert_eq!(
                maybe_eq(
                    &Err::<MaybeEqFoo, _>(MaybeEqErr),
                    &Err::<MaybeEqFoo, _>(MaybeEqErr),
                ),
                Some(true)
            );

            assert_eq!(
                maybe_eq(
                    &Ok::<_, MaybeEqErr>(NotMaybeEqFoo),
                    &Ok::<_, MaybeEqErr>(NotMaybeEqFoo),
                ),
                None
            );
            assert_eq!(
                maybe_eq(
                    &Ok::<_, MaybeEqErr>(NotMaybeEqFoo),
                    &Err::<NotMaybeEqFoo, _>(MaybeEqErr),
                ),
                None
            );
            assert_eq!(
                maybe_eq(
                    &Err::<NotMaybeEqFoo, _>(MaybeEqErr),
                    &Err::<NotMaybeEqFoo, _>(MaybeEqErr),
                ),
                None
            );
        }

        #[test]
        fn vec_maybe_eq() {
            let v1 = vec![MaybeEqFoo(1), MaybeEqFoo(2)];
            let v2 = vec![MaybeEqFoo(1), MaybeEqFoo(2)];
            let v3 = vec![MaybeEqFoo(3)];

            assert_eq!(maybe_eq(&v1, &v2), Some(true));
            assert_eq!(maybe_eq(&v1, &v3), Some(false));
            assert_eq!(maybe_eq(&v3, &v3), Some(true));

            let v4 = vec![NotMaybeEqFoo, NotMaybeEqFoo];

            assert_eq!(maybe_eq(&v4, &v4), None);
        }

        #[test]
        fn option_maybe_eq() {
            let o1 = Some(MaybeEqFoo(0));
            let o2 = Some(MaybeEqFoo(1));
            let o3: Option<MaybeEqFoo> = None;

            assert_eq!(maybe_eq(&o1, &o1), Some(true));
            assert_eq!(maybe_eq(&o1, &o2), Some(false));
            assert_eq!(maybe_eq(&o3, &o3), Some(true));
            assert_eq!(maybe_eq(&o2, &o3), Some(false));

            let o4 = Some(NotMaybeEqFoo);
            let o5: Option<NotMaybeEqFoo> = None;

            assert_eq!(maybe_eq(&o4, &o4), None);
            assert_eq!(maybe_eq(&o5, &o5), None);
            assert_eq!(maybe_eq(&o4, &o5), None);
        }
    }
}
