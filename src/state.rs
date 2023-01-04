use rand::Rng;

use crate::constants::ALPHABET;

pub struct State {
    pub should_quit: bool,
    pub game_result: Option<&'static str>,
    pub count: i32,
    pub guessed: Vec<char>,
    pub tries_left: u8,
    pub answer: Vec<char>,
}
impl State {
    pub fn new() -> State {
        State {
            should_quit: false,
            game_result: None,
            count: 0,
            tries_left: 6,
            guessed: Vec::new(),
            answer: get_random_answer(),
        }
    }
    pub fn guess(&mut self, new_char: char) {
        let new_char = new_char.to_ascii_uppercase();
        if ALPHABET.contains(&new_char) && !self.guessed.contains(&new_char) {
            self.guessed.push(new_char);
            if !self.answer.contains(&new_char) && self.tries_left > 0 {
                self.tries_left -= 1;
            }
        }
    }
}

fn get_random_answer() -> Vec<char> {
    let bytes = include_bytes!("answers.txt");
    let lines = std::str::from_utf8(bytes)
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let random_index = rand::thread_rng().gen_range(0..lines.len());
    lines[random_index]
        .chars()
        .map(|c| c.to_ascii_uppercase())
        .collect::<Vec<char>>()
}
