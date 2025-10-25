use clap::Parser;
use core::net::Ipv4Addr;
use macaddr::MacAddr6;
use wolrus::wake_on_lan;

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
    #[arg(short = 'i', long, default_value_t = Ipv4Addr::BROADCAST)]
    pub ip: Ipv4Addr,

    /// Target port; usually 0, 7 (Echo), or 9 (Discard)
    #[arg(short = 'p', long, default_value_t = 9)]
    pub port: u16,
}

fn main() {
    let args = Args::parse();

    let ip = args.ip.octets();
    let mac = args.mac.into_array();

    if let Err(err) = wake_on_lan(mac, Some(ip), Some(args.port)) {
        eprintln!("{err:?}");
    }
}
