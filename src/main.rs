mod app;
mod constants;
mod state;
mod widgets;
use crate::app::start;

fn main() -> crossterm::Result<()> {
    start()?;
    Ok(())
}
