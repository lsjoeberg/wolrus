# wolrus

A tiny Rust `no_std` library for [Wake on LAN].

## Usage

The library provides a single function `wake_on_lan`. The optional feature
`cli` installs a binary that can issue Wake-on-LAN commands from the terminal.

### Library

```sh
cargo add --git https://github.com/lsjoeberg/wolrus
```

```rust
use wolrus::wake_on_lan;

fn main() -> Result<(), wolrus::Error> {
    // Broadcast WoL on the local network.
    let mac_addr = [0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff];
    wake_on_lan(mac_addr, None, None)?;

    // Broadcast WoL on the local subnet.
    let ip = [192, 168, 0, 255];
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
