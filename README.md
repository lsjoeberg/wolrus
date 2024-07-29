# RCON

A tiny Rust library for [Wake on LAN].

## Usage

The library provides a single function `wake_on_lan`. The optional feature 
`cli` installs a binary that can issue Wake-on-LAN commands from the terminal.

### Library

```sh
cargo add --git https://github.com/lsjoeberg/wolrus
```

```rust
use std::net::Ipv4Addr;
use macaddr::MacAddr6;
use wolrus::wake_on_lan;

fn main() -> Result<(), std::io::Error> {
    // Broadcast WoL on the local network.
    let mac = MacAddr6::from([0, 1, 2, 3, 4, 5]);
    wake_on_lan(mac, None, None)?;

    // Broadcast WoL on the local subnet.
    let ip = Ipv4Addr::new(192, 168, 0, 255);
    wake_on_lan(mac, Some(ip), None)?;

    Ok(())
}
```

### CLI

The `cli` feature flag installs a binary `wolrus`:

```sh
cargo install -F cli --git https://github.com/lsjoeberg/wolrus
```

<!--References-->
[Wake on LAN]: http://en.wikipedia.org/wiki/Wake-on-LAN
