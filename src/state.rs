pub struct State {
    pub should_quit: bool,
    pub count: i32,
}

impl State {
    pub fn new() -> State {
        State {
            should_quit: false,
            count: 0,
        }
    }
}
