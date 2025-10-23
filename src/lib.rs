#![no_std]
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
//! use wolrus::wake_on_lan;
//!
//! // Broadcast WoL on the local network.
//! let mac = [0, 1, 2, 3, 4, 5];
//! wake_on_lan(mac, None, None).expect("failed to send packet");
//!
//! // Broadcast WoL on the local subnet.
//! wake_on_lan(mac, Some([192, 168, 0, 255]), None).expect("failed to send packet");
//! ```
//! [Limitations]: https://en.wikipedia.org/wiki/Wake-on-LAN#Magic_packet

use managed::ManagedSlice;
use smoltcp::socket::udp::{
    BindError as UdpBindError, PacketBuffer as UdpPacketBuffer,
    PacketMetadata as UdpPacketMetadata, SendError as UdpSendError, Socket as UdpSocket,
};
use smoltcp::wire::{IpAddress, IpEndpoint, Ipv4Address};

/// Default destination IP is the broadcast address: `255.255.255.255`.
pub const DEFAULT_ADDR: IpAddress = IpAddress::Ipv4(Ipv4Address::BROADCAST);

/// Default destination port is the Discard port: `9`.
pub const DEFAULT_PORT: u16 = 9;

/// Local address to bind the UPD socket to.
const BIND_ADDR: IpAddress = IpAddress::Ipv4(Ipv4Address::UNSPECIFIED);

/// Magic packet length in number of bytes.
const MAGIC_PACKET_LENGTH: usize = 102; // 6 + 6 * 16 = 102

#[derive(Debug)]
pub enum Error {
    BindError(UdpBindError),
    SendError(UdpSendError),
}

/// Build a magic Wake-on-LAN packet from a 48-bit MAC address.
#[inline]
fn build_magic_packet(mac: [u8; 6]) -> [u8; MAGIC_PACKET_LENGTH] {
    // The first 6 bytes if the packet bytes are all 0xff, followed by 16
    // repetitions of the 6-byte MAC address.
    let mut packet = [0xff; MAGIC_PACKET_LENGTH];

    // SAFETY: The slice length is constructed as a multiple of 6-byte arrays, 17 to be exact.
    let chunks = unsafe { packet.as_chunks_unchecked_mut() };

    // Fill the packet array with repetitions of the MAC-address, except the first 6 bytes.
    // TODO: Make fn const when feature `const_slice_make_iter` is stabilised.
    for chunk in chunks.iter_mut().skip(1) {
        *chunk = mac;
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
pub fn wake_on_lan(mac: [u8; 6], ip: Option<[u8; 4]>, port: Option<u16>) -> Result<(), Error> {
    // Set destination endpoint.
    let addr = match ip {
        Some(ip) => IpAddress::Ipv4(ip.into()),
        None => DEFAULT_ADDR,
    };
    let port = port.unwrap_or(DEFAULT_PORT);
    let remote_endpoint = IpEndpoint::new(addr, port);

    // Create UDP socket.
    let rx_buffer = UdpPacketBuffer::new([UdpPacketMetadata::EMPTY; 4], [0u8; 0]); // no receive
    let tx_storage = ManagedSlice::Borrowed(&mut [0u8; MAGIC_PACKET_LENGTH]);
    let tx_buffer = UdpPacketBuffer::new([UdpPacketMetadata::EMPTY; 4], tx_storage);
    let mut socket = UdpSocket::new(rx_buffer, tx_buffer);

    // Bind socket to a local endpoint.
    socket
        .bind(IpEndpoint::new(BIND_ADDR, 12345))
        .map_err(Error::BindError)?;

    // Send WOL magic packet.
    let packet = build_magic_packet(mac);
    socket
        .send_slice(&packet, remote_endpoint)
        .map_err(Error::SendError)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::build_magic_packet;
    use heapless::Vec;

    // A WoL packet is 6 bytes of 0xff, followed by 16 repetitions of the
    // 48-bit (6 byte) MAC address. This is 6 + 6 * 16 = 102 bytes.
    const EXPECTED_LEN: usize = 102;

    #[test]
    fn build_packet_bc() {
        let mac = [0xff; 6]; // broadcast
        let expected = [0xffu8; EXPECTED_LEN];

        let packet = build_magic_packet(mac);

        assert_eq!(packet.len(), EXPECTED_LEN);
        assert_eq!(packet.as_slice(), expected.as_slice());
    }

    #[test]
    fn build_packet_real() {
        let mac = [0, 1, 2, 3, 4, 5];
        let mut expected: Vec<u8, EXPECTED_LEN> = Vec::new();
        expected
            .extend_from_slice(&[0xff; 6])
            .expect("should fit in capacity");
        expected
            .extend_from_slice(mac.repeat(16).as_slice())
            .expect("should fit in capacity");

        let packet = build_magic_packet(mac);

        assert_eq!(packet.len(), EXPECTED_LEN);
        assert_eq!(packet.as_slice(), expected.as_slice());
    }
}
