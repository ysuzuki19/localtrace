// SPDX-License-Identifier: MPL-2.0

use super::Result;

pub fn catch_with_trace<F>(f: F)
where
    F: FnOnce() -> Result<()>,
{
    if let Err(e) = f() {
        eprintln!("{e}");
    }
}
