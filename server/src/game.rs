enum State {
    IDLE,
    NotStarted,
    Started,
}

pub(crate) struct Game {
    state: State,
}

impl Game {
    pub fn new() -> Self {
        Self { state: State::IDLE }
    }

    pub fn create_room(&mut self) {
        self.state = State::NotStarted;
    }
}
