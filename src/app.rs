use std;

use crossterm::{
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};

use crate::{state::State, widgets::get_letters_paragraph};

pub fn start() -> crossterm::Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut state = State::new();

    terminal.draw(|f| draw(f, &mut state))?;
    while !state.should_quit {
        terminal.draw(|f| update(f, &mut state))?;
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    Ok(())
}

fn update<B: Backend>(frame: &mut Frame<B>, state: &mut State) {
    let _ = read_event(state);
    draw(frame, state);
}
fn draw<B: Backend>(frame: &mut Frame<B>, state: &mut State) {
    let paragraph = get_letters_paragraph(&state);
    frame.render_widget(paragraph, frame.size());
}

fn read_event(state: &mut State) -> crossterm::Result<()> {
    match read()? {
        Event::Key(e) => handle_key_event(e, state),
        _ => (),
    }
    Ok(())
}

fn handle_key_event(e: KeyEvent, state: &mut State) {
    match (e.code, e.modifiers) {
        (KeyCode::Char('c'), crossterm::event::KeyModifiers::CONTROL) => state.should_quit = true,
        (KeyCode::Up, _) => state.count += 1,
        (KeyCode::Down, _) => state.count -= 1,
        (KeyCode::Char(c), _) => state.guess(c),
        _ => (),
    }
}
