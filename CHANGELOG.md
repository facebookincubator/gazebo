# Gazebo

## 0.2.0 (March 22, 2020)

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
