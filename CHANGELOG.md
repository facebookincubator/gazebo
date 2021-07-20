# Gazebo

## 0.4.1 (Jul 20, 2021)

* Allow deriving `Coerce` of newtypes to deal with lifetimes.
* Add `copied` to the `prelude` slice/`Vec` extensions.
* Add `try_unzip` to the `prelude` `Iter` extensions.

## 0.4.0 (Jul 15, 2021)

* Breaking change: `cloned` and `duped` on the `prelude` `Vec` extensions now operate on references rather than data. For the previous uses you should use the standard `to_vec()` method instead.
* Add the `AsARef` trait, similar to `AsRef` but working over both data and `RefCell`.
* Extend `Coerce` with more instances, support for `?Sized` types and a derivation.
* Add `try_map` to the `prelude` `Option` extensions.
* Add `dupe_from_slice` to the `prelude` `Vec` extensions.
* Add `TEq::teq_mut` to convert equal mutable pointers.
* Produce better error messages when derivations are used in inappropriate settings.

## 0.3.3 (Jul 6, 2021)

* Add `Coerce` and `CoerceKey` to implement zero-cost conversions between types with identical representations.

## 0.3.2 (Jun 20, 2021)

* Rename `OptionExt` to `OptionRefExt` (not a breaking change provided you `use gazebo::prelude::*`, as per the recommendation).
* Add `into_try_map` for `Option`.

## 0.3.1 (Jun 11, 2021)

* Optimise the internal implementation of `ARef`.
* Add `ARef::filter_map`, mirroring `Ref::filter_map`.
* Add `transmute_unchecked` for transmute with less static checks.
* Add `Hashed` to precompute the hash of a type.

## 0.3.0 (May 21, 2021)

* Breaking change: Make the internal structure of `ARef` abstract, introducing `ARef::new_ptr` and `ARef::new_ref` to create an `ARef`.

## 0.2.2 (May 10, 2021)

* Add `Dupe` for most `Copy` types from `std`, namely `&X`, `*const X`, `*mut X`, `Bound`, `Pin`, `NonNull`, `Poll`, `TypeId`, `PhantomPinned`, `Ipv4Addr`, `Ipv6Addr`, `SocketAddrV4`, `SocketAddrV6`, `ThreadId`, `SystemTime`.

## 0.2.1 (April 21, 2021)

* Add `Dupe` for `NonZero` types.
* Add an implementation of `AnyLifetime` for `str`.
* Implement traits on `ARef`, specifically `Display`, `Eq`, `Ord`, `PartialEq`, `PartialOrd`.

## 0.2.0 (March 22, 2021)

* Breaking change: Rename `clones` to `cloned`, `dupes` to `duped` and `owns` to `owned` - to follow Rust conventions better.
* Add `Dupe` for `fn()` (up to arity 12).
* Add macros for chained comparison, see `eq_chain!` and `cmp_chain!`.
* Add the `OptionExt` extension trait, containing `duped`.
* Add the `IterExt` extension trait, containing `try_all`, `try_any`, `try_eq_by`, `try_cmp_by` and `duped`.
* Introduce the `UnpackVariants` trait, to unpack the values inside an `enum`.
* Allow `any_lifetime!(&T)` to work, and add an instance for `&str`.
* Deprecate `trim_start_match_opt` and `trim_end_match_opt`. Use the functions `strip_prefix` and `strip_suffix` introduced in Rust 1.45.0.

## 0.1.0 (October 9, 2020)

* Initial version.
