pub use crate::error::Error;
pub use crate::structures::*;

pub type Result<T> = core::result::Result<T, Error>;
pub const PRIMARY_COLOR: termcolor::Color = termcolor::Color::Rgb(51, 202, 214);
