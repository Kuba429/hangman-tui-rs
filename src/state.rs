pub struct State {
    pub should_quit: bool,
    pub count: i32,
    pub guessed: Vec<char>,
}
const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
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
        let new_char = new_char.to_ascii_uppercase();
        if ALPHABET.contains(&new_char) && !self.guessed.contains(&new_char) {
            self.guessed.push(new_char);
        }
    }
}
