# PacketProbe

A lightning-fast port scanning CLI tool written in Rust that allows you to scan IP addresses using multiple threads.

## 📋 Prerequisites

- Rust (1.70.0 or later)


## Installation

### Install Rust and Cargo
If you don't have Rust installed, install it using rustup:

```bash
# On macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# On Windows
# Download rustup-init.exe from https://rustup.rs
```

### Install PacketProbe

```bash
# Clone the repository
git clone https://github.com/yourusername/packetprobe
cd packetprobe

# Build the project
cargo build --release

# Optional: Add to your path (macOS/Linux)
sudo ln -s $(pwd)/target/release/packetprobe /usr/local/bin/
```

## Features

- Multi-threaded port scanning for improved performance
- Simple and intuitive command-line interface
- Support for both IPv4 and IPv6 addresses
- Configurable thread count for optimal scanning

## Usage

```bash
# Basic usage with default settings (4 threads)
packetprobe 192.168.1.1

# Scan with custom thread count
packetprobe -j 8 192.168.1.1

# Display help
packetprobe -h
```

## 🔧 Command Line Arguments

- `<IP_ADDRESS>` - Direct IP address to scan
- `-j <THREADS> <IP_ADDRESS>` - Specify number of threads and IP address
- `-h` or `-help` - Display help information

## 💡 Examples

```bash
# Scan localhost
./packetprobe 127.0.0.1

# Scan with 16 threads
./packetprobe -j 16 192.168.0.1

# Scan IPv6 address
./packetprobe ::1
```

## 🧪 Development

```bash
# Run tests
cargo test

# Run with debug output
cargo run -- 127.0.0.1

# Build with optimizations
cargo build --release
```
