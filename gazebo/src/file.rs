/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

//! File/IO operations.

use std::{
    fs::{create_dir_all, write},
    io,
    path::Path,
};

/// A simple api for creating all the directories up to a path for a file, and
/// then writing the contents to that file.
pub fn create_dirs_and_write<P: AsRef<Path>, C: AsRef<[u8]>>(
    path: P,
    contents: C,
) -> io::Result<()> {
    // no parent means no directory component, and we can directly write to that
    // file
    path.as_ref().parent().map_or(Ok(()), create_dir_all)?;

    write(path.as_ref(), contents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::Read};

    #[test]
    fn test_create_all_and_write() -> io::Result<()> {
        let temp = std::env::temp_dir();
        let path = temp.join("foo/bar");
        create_dirs_and_write(&path, "contents")?;

        let mut contents = String::new();
        File::open(&path)?.read_to_string(&mut contents)?;
        assert_eq!(contents, "contents");

        Ok(())
    }
}
