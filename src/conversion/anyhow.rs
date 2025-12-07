use crate::Error;

impl Error {
    pub fn from_anyhow(err: anyhow::Error) -> Self {
        Error::new(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;

    #[test]
    fn test_from_anyhow_error() {
        let anyhow_err: anyhow::Error = anyhow::anyhow!("test error from anyhow");
        let localtrace_err: Error = Error::from_anyhow(anyhow_err);
        assert!(localtrace_err
            .to_string()
            .contains("test error from anyhow"));
    }

    #[test]
    fn test_from_anyhow_with_context() {
        fn may_fail() -> anyhow::Result<()> {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "file not found",
            ))?
        }

        let anyhow_err = may_fail().context("failed to process file").unwrap_err();
        let localtrace_err: Error = Error::from_anyhow(anyhow_err);
        assert!(localtrace_err
            .to_string()
            .contains("failed to process file"));
    }
}
