use std::io;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

use macaddr::MacAddr6;

/// A wake-on-lan magic packet starts with a fixed payload: 6 bytes of all 255.
const MAGIC_PAYLOAD: [u8; 6] = [0xff; 6];

fn build_magic_packet(mac: MacAddr6) -> Vec<u8> {
    [&MAGIC_PAYLOAD, mac.as_bytes().repeat(16).as_slice()].concat()
}

/// Send wake-on-lan packet.
pub fn wake_on_lan(ip: Ipv4Addr, port: u16, mac: MacAddr6) -> Result<(), io::Error> {
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
