use crate::constants::ALPHABET;

pub struct State {
    pub should_quit: bool,
    pub count: i32,
    pub guessed: Vec<char>,
    pub answer: Vec<char>,
}
impl State {
    pub fn new() -> State {
        State {
            should_quit: false,
            count: 0,
            guessed: Vec::new(),
            answer: String::from("elo elo some phrase")
                .chars()
                .map(|c| c.to_ascii_uppercase())
                .collect::<Vec<char>>(),
        }
    }
    pub fn guess(&mut self, new_char: char) {
        let new_char = new_char.to_ascii_uppercase();
        if ALPHABET.contains(&new_char) && !self.guessed.contains(&new_char) {
            self.guessed.push(new_char);
        }
    }
}
