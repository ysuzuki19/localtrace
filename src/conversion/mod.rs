use std::result;

#[cfg(feature = "anyhow")]
pub mod anyhow;

pub trait ToLocaltrace<T> {
    fn to_localtrace(self) -> result::Result<T, crate::Error>;
}
