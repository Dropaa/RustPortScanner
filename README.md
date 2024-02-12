# Port Scanner

A simple port scanner written in Rust that allows you to scan ports on a target IP address. This project was created for learning purposes.

## Features

- Scan a range of ports on a specified IP address.
- Perform stealth scans to avoid detection.
- Detect services running on common ports (e.g., HTTP, HTTPS).

## Installation

To use the port scanner, make sure you have Rust and Cargo installed. If not, you can install them from [https://www.rust-lang.org/](https://www.rust-lang.org/).

Clone the repository:

```bash
git clone [https://github.com/yourusername/port-scanner.git]
```

Navigate to the project directory:

```bash
cd port-scanner
```

Compile the project:

```bash
cargo build --release
```

## Usage

Run the compiled binary with the following command:

```bash
./target/release/port-scanner [OPTIONS]
```

### Options

- `--ip <IP_ADDRESS>`: Specify the target IP address to scan (required).
- `--ports <PORTS>`: Specify one or more ports to scan, separated by commas (e.g., `80,443`).
- `--start-port <START_PORT>`: Specify the starting port for the scan (default is `1`).
- `--end-port <END_PORT>`: Specify the ending port for the scan (default is `1024`).
- `--stealth`: Perform a stealth scan to avoid detection.
- `--service`: Perform service detection to identify services running on common ports.

### Examples

Scan ports 1 to 1024 on the IP address 192.168.1.100:

```bash
./target/release/port-scanner --ip 192.168.1.100
```

Scan ports 80 and 443 on the IP address 192.168.1.100 using a stealth scan:

```bash
./target/release/port-scanner --ip 192.168.1.100 --ports 80,443 --stealth
```

Scan ports 1 to 1024 on the IP address 192.168.1.100 and perform service detection:

```bash
./target/release/port-scanner --ip 192.168.1.100 --service
```
