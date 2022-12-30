use std::{self, io};

use crossterm::{
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    style::Style,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

use crate::state::State;

pub fn start() -> Result<(), io::Error> {
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
    let paragraph_content = state
        .guessed
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let paragraph_block = Block::default().borders(Borders::ALL).style(
        Style::default()
            .bg(tui::style::Color::DarkGray)
            .fg(tui::style::Color::LightGreen),
    );
    frame.render_widget(
        Paragraph::new(paragraph_content)
            .alignment(tui::layout::Alignment::Center)
            .block(paragraph_block),
        frame.size(),
    );
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
        (KeyCode::Char(c), _) => state.guessed.push(c),
        _ => (),
    }
}
