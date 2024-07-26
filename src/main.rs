use std::io;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

use macaddr::MacAddr6;

mod cli;

const MAGIC_PAYLOAD: [u8; 6] = [0xff; 6];

fn build_magic_packet(mac: MacAddr6) -> Vec<u8> {
    [&MAGIC_PAYLOAD, mac.as_bytes().repeat(16).as_slice()].concat()
}

fn send_wol(ip: Ipv4Addr, port: u16, mac: MacAddr6) -> Result<(), io::Error> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // Permits sending of broadcast messages.
    socket.set_broadcast(true)?;

    // Connect to target host.
    let target = SocketAddrV4::new(ip, port);
    socket.connect(target)?;

    // Send WOL magic packet.
    let packet = build_magic_packet(mac);
    socket.send(&packet)?;
    Ok(())
}

fn main() {
    let args = cli::Args::get();
    if let Err(err) = send_wol(args.ip, args.port, args.mac) {
        eprintln!("{err}");
    };
}
