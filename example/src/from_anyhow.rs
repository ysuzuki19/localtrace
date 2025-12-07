// SPDX-License-Identifier: MIT
use localtrace::{catch_with_trace, Error, Result};

fn anyhow_operation() -> anyhow::Result<()> {
    anyhow::bail!("something went wrong in anyhow")
}

fn wrapped() -> Result<()> {
    let result = anyhow_operation();
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::from_anyhow(e)),
    }
}

fn main() {
    catch_with_trace(|| {
        wrapped()?;
        Ok(())
    });
}
