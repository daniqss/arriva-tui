use std::io::{self, stdout, Stdout};

use crossterm::{execute, terminal::*};
use ratatui::prelude::*;
use crate::prelude::*;

/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal
pub fn init() -> Result<Tui> {
    print!("{}[2J", 27 as char);
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Ok(Terminal::new(CrosstermBackend::new(stdout()))?)
}

/// Restore the terminal to its original state
pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
