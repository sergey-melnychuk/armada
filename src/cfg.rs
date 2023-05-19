use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Config {
    pub poll_delay: Duration,
    pub ethereum_contract_address: String,
}

impl Config {
    pub fn new(poll_delay: Duration, ethereum_contract_address: String) -> Self {
        Self {
            poll_delay,
            ethereum_contract_address,
        }
    }
}
