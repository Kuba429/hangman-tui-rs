pub struct State {
    pub should_quit: bool,
    pub count: i32,
    pub guessed: Vec<char>,
}
const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
impl State {
    pub fn new() -> State {
        State {
            should_quit: false,
            count: 0,
            guessed: Vec::new(),
        }
    }
    pub fn guess(&mut self, new_char: char) {
        if ALPHABET.contains(&new_char) && !self.guessed.contains(&new_char) {
            self.guessed.push(new_char);
        }
    }
}
