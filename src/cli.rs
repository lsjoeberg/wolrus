use std::net::Ipv4Addr;

use clap::Parser;
use macaddr::MacAddr6;

const DEFAULT_ADDR: Ipv4Addr = Ipv4Addr::BROADCAST;
const DEFAULT_PORT: u16 = 9;

#[derive(Parser, Debug)]
pub struct Args {
    /// Target NIC 48-bit MAC address
    #[arg()]
    pub mac: MacAddr6,
    /// Target IP address
    #[arg(short = 'i', default_value_t = DEFAULT_ADDR)]
    pub ip: Ipv4Addr,
    /// Target port; usually 0, 7, or 9
    #[arg(short = 'p', default_value_t = DEFAULT_PORT)]
    pub port: u16,
}

impl Args {
    pub fn get() -> Self {
        Self::parse()
    }
}
