# 🔍 PacketProbe

A lightning-fast port scanning CLI tool written in Rust that allows you to scan IP addresses using multiple threads.

## ⚡ Features

- Multi-threaded port scanning for improved performance
- Simple and intuitive command-line interface
- Support for both IPv4 and IPv6 addresses
- Configurable thread count for optimal scanning

## 🚀 Usage

```bash
# Basic usage with default settings (4 threads)
packetprobe 192.168.1.1

# Scan with custom thread count
packetprobe -j 8 192.168.1.1

# Display help
packetprobe -h