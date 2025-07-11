//! Send Wake-on-LAN packets.
//!
//! The Wake-on-LAN implementation has the following [limitations]:
//! - May not work outside the local network.
//! - Requires hardware support in destination computer.
//! - Most 802.11 wireless interfaces do not maintain a link in low-power
//!   states and cannot receive a magic packet.
//!
//! # Examples
//!
//! ```rust
//! use std::net::Ipv4Addr;
//! use macaddr::MacAddr6;
//! use wolrus::wake_on_lan;
//!
//! // Broadcast WoL on the local network.
//! let mac = MacAddr6::from([0, 1, 2, 3, 4, 5]);
//! wake_on_lan(mac, None, None).expect("failed to send packet");
//!
//! // Broadcast WoL on the local subnet.
//! let ip = Ipv4Addr::new(192, 168, 0, 255);
//! wake_on_lan(mac, Some(ip), None).expect("failed to send packet");
//! ```
//! [Limitations]: https://en.wikipedia.org/wiki/Wake-on-LAN#Magic_packet

use std::io;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

use macaddr::MacAddr6;

/// Default target IP is the broadcast address: `255.255.255.255`.
pub const DEFAULT_ADDR: Ipv4Addr = Ipv4Addr::BROADCAST;

/// Default target port is the Discard port: `9`.
pub const DEFAULT_PORT: u16 = 9;

/// Magic packet length in number of bytes.
const MAGIC_PACKET_LENGTH: usize = 102; // 6 + 6 * 16 = 102

/// Build a magic Wake-on-LAN packet from a 48-bit MAC address.
fn build_magic_packet(mac: MacAddr6) -> [u8; MAGIC_PACKET_LENGTH] {
    // The first 6 bytes if the packet are all 0xff, followed by 16
    // repetitions of the 6-byte MAC address.
    let mut packet = [0xff; MAGIC_PACKET_LENGTH];
    let mac_chunk = mac.into_array();

    // SAFETY: The slice length is constructed as a multiple of 6-byte arrays, 17 to be exact.
    let chunks = unsafe { packet.as_chunks_unchecked_mut() };

    // Fill the packet array with repetitions of the MAC-address, except the first 6 bytes.
    // TODO: Make fn const when feature `const_slice_make_iter` is stabilised.
    for chunk in chunks.iter_mut().skip(1) {
        *chunk = mac_chunk;
    }
    packet
}

/// Send a Wake-on-LAN packet over UDP.
///
/// The function creates a UDP socket bound to `0.0.0.0:0` and sends a
/// Wake-on-LAN UDP datagram to the specified `ip` and `port`, or default
/// `255.255.255.255` on port `9`.
///
/// # Errors
/// Will return `Err` if the OS is unable to create a socket.
pub fn wake_on_lan(
    mac: MacAddr6,
    ip: Option<Ipv4Addr>,
    port: Option<u16>,
) -> Result<(), io::Error> {
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;

    // Permits sending of broadcast messages.
    socket.set_broadcast(true)?;

    // Connect to target host.
    let target = SocketAddrV4::new(ip.unwrap_or(DEFAULT_ADDR), port.unwrap_or(DEFAULT_PORT));
    socket.connect(target)?;

    // Send WOL magic packet.
    let packet = build_magic_packet(mac);
    socket.send(&packet)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use macaddr::MacAddr6;

    use crate::build_magic_packet;

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
        expected.extend_from_slice(&[0xff; 6]);
        expected.extend_from_slice(mac.as_bytes().repeat(16).as_slice());

        let packet = build_magic_packet(mac);

        assert_eq!(packet.len(), EXPECTED_LEN);
        assert_eq!(packet.as_slice(), expected.as_slice());
    }
}
