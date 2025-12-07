// SPDX-License-Identifier: MIT
use localtrace::{catch_with_trace, Result, ToLocaltrace};

fn anyhow_operation() -> anyhow::Result<()> {
    anyhow::bail!("something went wrong in anyhow")
}

fn wrapped() -> Result<()> {
    anyhow_operation().to_localtrace()?;
    Ok(())
}

fn main() {
    catch_with_trace(|| {
        wrapped()?;
        Ok(())
    });
}
