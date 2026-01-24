use crossterm::{cursor::MoveTo, execute, terminal::{self, Clear}};
use std::io;

pub fn clear_screen() {
    execute!(
        io::stdout(),
        Clear(terminal::ClearType::All),
        MoveTo(0,0)
    ).unwrap();
}
pub fn reset_terminal() {
    execute!(
        io::stdout(),
        Clear(terminal::ClearType::All),
        Clear(terminal::ClearType::Purge),
        MoveTo(0,0)
    ).unwrap();
}
