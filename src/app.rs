use std;

use crossterm::{
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::Block,
    Frame, Terminal,
};

use crate::{
    state::State,
    widgets::{
        get_answer_paragraph, get_available_letters_paragraph, get_chunks, get_hangman_widget,
        get_universal_border_type, get_universal_style,
    },
};

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
    let chunks = get_chunks(vec![20, 20, 50, 10], frame.size());
    // filler block
    frame.render_widget(
        Block::default()
            .border_type(get_universal_border_type())
            .style(get_universal_style()),
        chunks[0],
    );
    // answer
    frame.render_widget(get_answer_paragraph(&state), chunks[1]);
    // hangman drawing
    frame.render_widget(get_hangman_widget(&state), chunks[2]);
    // available letters
    frame.render_widget(get_available_letters_paragraph(&state), chunks[3]);
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
