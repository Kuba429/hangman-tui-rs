use tui::{
    style::Style,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{constants::ALPHABET, state::State};

pub fn get_universal_style() -> Style {
    Style::default()
        .bg(tui::style::Color::White)
        .fg(tui::style::Color::Black)
}
pub fn get_universal_border_type() -> BorderType {
    BorderType::Rounded
}

pub fn get_letters_paragraph(state: &State) -> Paragraph {
    let paragraph_content = ALPHABET
        .iter()
        .map(|i| {
            return if state.guessed.contains(i) {
                i.to_string()
            } else {
                String::from("_")
            };
        })
        .collect::<Vec<String>>()
        .join(" ");
    let paragraph_block = Block::default()
        .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
        .border_type(get_universal_border_type())
        .style(get_universal_style());
    return Paragraph::new(paragraph_content)
        .alignment(tui::layout::Alignment::Center)
        .block(paragraph_block);
}

pub fn get_hangman_widget() -> Paragraph<'static> {
    let block = Block::default()
        .style(get_universal_style())
        .border_type(get_universal_border_type())
        .borders(Borders::LEFT | Borders::RIGHT);
    Paragraph::new("Hangman")
        .alignment(tui::layout::Alignment::Center)
        .block(block)
}

pub fn default_block() -> Block<'static> {
    Block::default()
        .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
        .border_type(get_universal_border_type())
        .style(get_universal_style())
}
