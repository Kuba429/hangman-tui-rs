pub struct State {
    pub should_quit: bool,
    pub count: i32,
    pub guessed: Vec<char>,
}

impl State {
    pub fn new() -> State {
        State {
            should_quit: false,
            count: 0,
            guessed: Vec::new(),
        }
    }
}
