/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

#![cfg_attr(feature = "str_pattern_extensions", feature(pattern))]
#![cfg_attr(feature = "str_pattern_extensions", feature(associated_type_bounds))]
#![allow(renamed_and_removed_lints)] // intra_doc_link_resolution_failure got renamed
#![deny(intra_doc_link_resolution_failure)]

//! A collection of well-tested primitives that have been useful. Most modules stand alone.

pub mod any;
pub mod cast;
pub mod cell;
pub mod cmp;
pub mod dupe;
pub(crate) mod ext;
pub mod file;
pub mod phantom;
pub mod prelude;
pub mod types;
pub mod variants;

#[cfg(test)]
mod test;

/// Causes Rust to exit the process when any panic occurs.
pub fn terminate_on_panic() {
    let orig_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        orig_hook(panic_info);
        std::process::exit(1);
    }));
}
