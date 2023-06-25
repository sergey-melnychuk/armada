use std::{net::SocketAddr, time::Duration};

pub struct Profile {
    pub network: String,
    pub eth_url: String,
    pub seq_url: String,
    pub eth_contract_address: String,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub network: String,
    pub rpc_bind_addr: SocketAddr,
    pub src_poll_delay: Duration,
    pub seq_poll_delay: Duration,
    pub eth_poll_delay: Duration,
    pub ethereum_contract_address: String,
}

impl Config {
    pub fn new(
        network: String,
        rpc_bind_addr: SocketAddr,
        src_poll_delay: Duration,
        seq_poll_delay: Duration,
        eth_poll_delay: Duration,
        ethereum_contract_address: String,
    ) -> Self {
        Self {
            network,
            rpc_bind_addr,
            src_poll_delay,
            seq_poll_delay,
            eth_poll_delay,
            ethereum_contract_address,
        }
    }
}
