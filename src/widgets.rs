use tui::{
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    constants::{ALPHABET, HANGMAN_STAGES},
    state::State,
};

pub fn get_universal_style() -> Style {
    Style::default().fg(tui::style::Color::White)
}
pub fn get_universal_border_type() -> BorderType {
    BorderType::Rounded
}

pub fn get_chunks(constraints: Vec<u16>, split: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .constraints(
            constraints
                .into_iter()
                .map(|c| Constraint::Percentage(c))
                .collect::<Vec<Constraint>>()
                .as_ref(),
        )
        .split(split)
}

pub fn get_answer_paragraph(state: &State) -> Paragraph {
    let paragraph_content = if state.game_result.is_none() {
        state
            .answer
            .iter()
            .map(|i| {
                return if state.guessed.contains(i) {
                    i.to_string()
                } else if *i == ' ' {
                    String::from(" ")
                } else {
                    String::from("_")
                };
            })
            .collect::<Vec<String>>()
            .join("")
    } else {
        state
            .answer
            .iter()
            .map(char::to_string)
            .collect::<Vec<String>>()
            .join("")
    };
    let paragraph_block = Block::default()
        .border_type(get_universal_border_type())
        .style(get_universal_style());
    return Paragraph::new(paragraph_content)
        .alignment(tui::layout::Alignment::Center)
        .block(paragraph_block);
}

pub fn get_hangman_widget(state: &State) -> Paragraph<'static> {
    let block = Block::default()
        .style(get_universal_style())
        .border_type(get_universal_border_type());
    Paragraph::new(HANGMAN_STAGES[state.tries_left as usize])
        .alignment(tui::layout::Alignment::Center)
        .block(block)
}

pub fn get_available_letters_paragraph(state: &State) -> Paragraph {
    let block = Block::default()
        .style(get_universal_style())
        .borders(Borders::ALL)
        .border_style(get_universal_style().fg(tui::style::Color::DarkGray))
        .title(if state.game_result.is_none() {
            "Available letters"
        } else {
            "Press any key to exit"
        })
        .border_type(get_universal_border_type());

    let paragraph_content = if let Some(r) = &state.game_result {
        r.to_string()
    } else {
        ALPHABET
            .iter()
            .filter(|l| !state.guessed.contains(&l))
            .collect::<String>()
    };

    Paragraph::new(paragraph_content)
        .block(block)
        .alignment(tui::layout::Alignment::Center)
}
