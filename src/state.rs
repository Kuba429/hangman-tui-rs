use std::{fs::File, io::Read};

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
    let mut lines = String::new();
    File::open("answers.txt")
        .expect("Can't read file with answers")
        .read_to_string(&mut lines)
        .expect("Can't read file with answers");
    let lines = lines
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let mut rng = rand::thread_rng();

    lines[rng.gen_range(0..lines.len())]
        .chars()
        .map(|c| c.to_ascii_uppercase())
        .collect::<Vec<char>>()
}
