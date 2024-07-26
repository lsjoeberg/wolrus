use std::net::Ipv4Addr;

use clap::Parser;
use macaddr::MacAddr6;

const DEFAULT_ADDR: Ipv4Addr = Ipv4Addr::BROADCAST;
const DEFAULT_PORT: u16 = 9;

/// Send wake-on-lan packets.
///
/// Limitations: may not work outside the local network; requires hardware
/// support in destination computer; most 802.11 wireless interfaces do not
/// maintain a link in low-power states and cannot receive a magic packet.
#[derive(Parser, Debug)]
pub struct Args {
    /// Target NIC 48-bit MAC address
    #[arg()]
    pub mac: MacAddr6,

    /// Target IP address
    ///
    /// Hint: For a NIC on a local subnet 192.168.10.0/24, use the subnet's
    /// broadcast address: 192.168.10.255.
    #[arg(short = 'i', long, default_value_t = DEFAULT_ADDR)]
    pub ip: Ipv4Addr,

    /// Target port; usually 0, 7 (Echo), or 9 (Discard)
    #[arg(short = 'p', long, default_value_t = DEFAULT_PORT)]
    pub port: u16,
}

impl Args {
    pub fn get() -> Self {
        Self::parse()
    }
}
