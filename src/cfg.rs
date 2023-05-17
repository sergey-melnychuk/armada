use std::time::Duration;

// TODO: configuration

#[derive(Clone, Debug)]
pub struct Config {
    pub poll_delay: Duration,
}

impl Config {
    pub fn new(poll_delay: Duration) -> Self {
        Self { poll_delay }
    }
}
