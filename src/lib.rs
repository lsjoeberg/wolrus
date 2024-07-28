use std::io;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

use macaddr::MacAddr6;

/// A wake-on-lan magic packet starts with a fixed payload: 6 bytes of all 255.
const MAGIC_PAYLOAD: [u8; 6] = [0xff; 6];

fn build_magic_packet(mac: MacAddr6) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::with_capacity(6 + 6 * 16);
    buf.extend_from_slice(&MAGIC_PAYLOAD);
    buf.extend_from_slice(mac.as_bytes().repeat(16).as_slice());
    buf
}

/// Send wake-on-lan packet.
pub fn wake_on_lan(ip: Ipv4Addr, port: u16, mac: MacAddr6) -> Result<(), io::Error> {
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;

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

#[cfg(test)]
mod tests {
    use macaddr::MacAddr6;

    use crate::{build_magic_packet, MAGIC_PAYLOAD};

    // A WoL packet is 6 bytes of 0xff, followed by 16 repetitions of the
    // 48-bit (6 byte) MAC address. This is 6 + 6 * 16 = 102 bytes.
    const EXPECTED_LEN: usize = 102;

    #[test]
    fn build_packet_bc() {
        let mac = MacAddr6::broadcast();
        let expected = [0xffu8; EXPECTED_LEN];

        let packet = build_magic_packet(mac);

        assert_eq!(packet.len(), EXPECTED_LEN);
        assert_eq!(packet.as_slice(), expected.as_slice());
    }

    #[test]
    fn build_packet_real() {
        let mac = MacAddr6::new(0, 1, 2, 3, 4, 5);
        let mut expected = Vec::with_capacity(EXPECTED_LEN);
        expected.extend_from_slice(&MAGIC_PAYLOAD);
        expected.extend_from_slice(mac.as_bytes().repeat(16).as_slice());

        let packet = build_magic_packet(mac);

        assert_eq!(packet.len(), EXPECTED_LEN);
        assert_eq!(packet.as_slice(), expected.as_slice());
    }
}
