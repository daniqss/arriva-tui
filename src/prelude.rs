pub use crate::error::Error;
pub use crate::structures::*;

pub type Result<T> = core::result::Result<T, Error>;
