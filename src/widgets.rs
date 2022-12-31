use tui::{
    style::Style,
    widgets::{Block, Borders, Paragraph},
};

use crate::{constants::ALPHABET, state::State};

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
    let paragraph_block = Block::default().borders(Borders::ALL).style(
        Style::default()
            .bg(tui::style::Color::DarkGray)
            .fg(tui::style::Color::LightGreen),
    );
    return Paragraph::new(paragraph_content)
        .alignment(tui::layout::Alignment::Center)
        .block(paragraph_block);
}
