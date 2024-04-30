pub use crate::error::Error;
pub use crate::structures::*;

pub type Result<T> = core::result::Result<T, Error>;

pub const PRIMARY_COLOR_TC: termcolor::Color = termcolor::Color::Rgb(51, 202, 214);
pub const PRIMARY_COLOR_RTT: ratatui::style::Color = ratatui::style::Color::Rgb(51, 202, 214);
pub const SECUNDARY_COLOR_TC: termcolor::Color = termcolor::Color::Rgb(245, 200, 118);
pub const SECUNDARY_COLOR_RTT: ratatui::style::Color = ratatui::style::Color::Rgb(245, 200, 118);
pub const ERROR_COLOR_TC: termcolor::Color = termcolor::Color::Rgb(255, 0, 0);
pub const ERROR_COLOR_RTT: ratatui::style::Color = ratatui::style::Color::Rgb(255, 0, 0);
pub const BG_COLOR_TC: termcolor::Color = termcolor::Color::Rgb(0, 0, 0);
pub const BG_COLOR_RTT: ratatui::style::Color = ratatui::style::Color::Rgb(0, 0, 0);

