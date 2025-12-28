# nt (Network Monitor) 🛰️

`nt` is a fast, minimal command-line utility written in Rust for monitoring active network connections on Linux. It provides a real-time view of TCP and UDP sockets, associated process IDs (PIDs), and program names.

## Features ✨

* **Protocol Filtering**: View TCP, UDP, or all connections.
* **Process Identification**: Automatically maps connections to their PIDs and program names using the `/proc` filesystem.
* **Automatic Sorting**: Connections are sorted alphabetically by program name for easy reading.
* **Watch Mode**: Use the `-w` flag to refresh the output automatically at a set interval.
* **Robust & Resilient**: Gracefully handles permission issues or short-lived processes without crashing.

## Installation 🛠️

### Prerequisites
* Linux OS (uses `/proc/net` and `/proc/[pid]`)
* Rust and Cargo installed

### Building from source
1. Clone the repository.
2. Build the release binary:
   ```bash
   cargo build --release