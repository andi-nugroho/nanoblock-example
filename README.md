# nano-block-example

⚠️ **WARNING: This project operates at the kernel level and can affect network connectivity. Use with caution and test in a safe environment first!**

A high-performance network firewall example using eBPF (Extended Berkeley Packet Filter) and the NanoBlock framework. This project demonstrates how to build a packet filtering system that can block IP addresses and allow specific ports at the kernel level for maximum performance.

## What it does

This example creates an XDP (eXpress Data Path) program that:

- Filters network packets at the kernel level for ultra-low latency
- Blocks specific IP addresses
- Allows traffic on specific ports (SSH on port 22, web on port 3000)
- Uses eBPF maps for dynamic rule management
- Provides a user-space control interface via the NanoBlock FirewallManager

## Prerequisites

1. **Rust toolchains:**

   ```bash
   rustup toolchain install stable
   rustup toolchain install nightly --component rust-src
   ```

2. **Cross-compilation tools (if needed):**

   ```bash
   # For cross-compiling to Linux from macOS
   rustup target add ${ARCH}-unknown-linux-musl
   brew install llvm
   brew install filosottile/musl-cross/musl-cross
   ```

3. **eBPF linker:**
   ```bash
   cargo install bpf-linker --no-default-features  # on macOS
   # or
   cargo install bpf-linker  # on Linux
   ```

## Usage

⚠️ **CAUTION: Always test in a safe environment first! This program can block network traffic and potentially lock you out of remote access.**

### Basic Usage

1. **Build and run the firewall:**

   ```bash
   cargo run --release
   ```

2. **Specify a network interface (default: bond0):**

   ```bash
   cargo run --release -- --iface eth0
   ```

   ⚠️ **Make sure you're not using the same interface you're connected through for remote access!**

### What happens when you run it

The program will:

1. Load the eBPF XDP program onto the specified network interface
2. Initialize the FirewallManager with default rules:
   - Allow SSH traffic on port 22
   - Allow web traffic on port 3000
   - Block IP address `3.33.52.3`
3. Display the status of blocked IPs
4. Wait for Ctrl-C to exit

### Example Output

```
FirewallManager initialized
FirewallManager init 22
FirewallManager init 3000
FirewallManager ip allowed 3.33.52.3
FirewallManager ip blocked 3.33.52.3 true
Waiting for Ctrl-C...
```

### Programmatic Usage

The example demonstrates the NanoBlock FirewallManager API:

```rust
let mut fw = FirewallManager::new(&mut ebpf)?;

// Allow specific ports
fw.allow_port(22).await?;   // SSH
fw.allow_port(3000).await?; // Web server

// Block specific IP addresses
let ip = IpAddr::from_str("192.168.1.100")?;
fw.block_ip(ip).await?;

// Check if an IP is blocked
let is_blocked = fw.is_ip_blocked(ip).await?;
```

## Cross-compiling on macOS

Cross compilation works on both Intel and Apple Silicon Macs:

```bash
CC=${ARCH}-linux-musl-gcc cargo build --package nano-block-example --release \
  --target=${ARCH}-unknown-linux-musl \
  --config=target.${ARCH}-unknown-linux-musl.linker=\"${ARCH}-linux-musl-gcc\"
```

The cross-compiled program `target/${ARCH}-unknown-linux-musl/release/nano-block-example` can be copied to a Linux server or VM and run there.

## Requirements

⚠️ **IMPORTANT SAFETY NOTES:**

- **Test environment first** - Never run this on a production system without thorough testing
- **Backup access** - Ensure you have alternative access methods (console, IPMI, etc.) in case network access is lost
- **Monitor carefully** - Watch system logs and network connectivity during testing

**Technical Requirements:**

- **Linux kernel 4.18+** (for XDP support)
- **Root privileges** (required for loading eBPF programs)
- **Network interface** that supports XDP (most modern interfaces do)

## Architecture

- **eBPF program** (`nano-block-example-ebpf/`): Kernel-space packet filtering
- **User-space program** (`nano-block-example/`): Control interface and rule management
- **Common library** (`nano-block-example-common/`): Shared types and utilities

## License

With the exception of eBPF code, nano-block-example is distributed under the terms
of either the [Apache License] (version 2.0), at your
option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

### eBPF

All eBPF code is distributed under either the terms of the
[GNU General Public License, Version 2] at your
option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the GPL-2 license, shall be
dual licensed as above, without any additional terms or conditions.

[Apache license]: LICENSE-APACHE
[MIT license]: LICENSE-MIT
[GNU General Public License, Version 2]: LICENSE-GPL2
