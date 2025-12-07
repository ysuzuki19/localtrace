use std::result;

use crate::{conversion::ToLocaltrace, Error};

impl<T> ToLocaltrace<T> for result::Result<T, anyhow::Error> {
    fn to_localtrace(self) -> result::Result<T, Error> {
        self.map_err(|e| Error::new(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;

    #[test]
    fn test_to_localtrace() {
        let anyhow_result: anyhow::Result<()> = Err(anyhow::anyhow!("test error from anyhow"));
        let localtrace_result = anyhow_result.to_localtrace();
        assert!(localtrace_result
            .unwrap_err()
            .to_string()
            .contains("test error from anyhow"));
    }

    #[test]
    fn test_to_localtrace_with_context() {
        fn may_fail() -> anyhow::Result<()> {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "file not found",
            ))?
        }

        let anyhow_result = may_fail().context("failed to process file");
        let localtrace_result = anyhow_result.to_localtrace();
        assert!(localtrace_result
            .unwrap_err()
            .to_string()
            .contains("failed to process file"));
    }

    #[test]
    fn test_to_localtrace_ok() {
        let anyhow_result: anyhow::Result<i32> = Ok(42);
        let localtrace_result = anyhow_result.to_localtrace();
        assert_eq!(localtrace_result.unwrap(), 42);
    }
}
