// SPDX-License-Identifier: MPL-2.0

pub fn catch_with_trace<F>(f: F)
where
    F: FnOnce() -> super::Result<()>,
{
    let res = f();
    assert!(
        res.is_ok(),
        "Function failed with error:\n {}",
        res.unwrap_err()
    );
}
