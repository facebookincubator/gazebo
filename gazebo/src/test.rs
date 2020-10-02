/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use crate::prelude::*;
use std::collections::HashMap;

enum NoDefault {
    _None,
}

#[derive(Default_)]
struct Foo<K, V, Z> {
    mp: HashMap<K, V>,
    xs: Vec<Z>,
}

#[derive(Default_, Debug, PartialEq)]
struct Bar;

#[test]
fn test_default_() {
    let x: Foo<NoDefault, NoDefault, NoDefault> = Default::default();
    assert_eq!(x.mp.len(), 0);
    assert_eq!(x.xs.len(), 0);

    assert_eq!(Bar, Default::default());
}
