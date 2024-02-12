use std::net::{IpAddr, Ipv4Addr, TcpListener, TcpStream}; 
use std::io::{Read, Write}; 
use std::thread; 
use clap::{App, Arg};
use serde::Serialize;

/// Structure representing the result of a port scan.
#[derive(Serialize)]
struct PortResult {
    port: u16,
    status: String,
    service: Option<String>, // Add the `service` field
}

/// Function to perform a scan on a single port of a given IP address.
///
/// # Arguments
///
/// * `ip` - The IP address to scan.
/// * `port` - The port number to scan.
/// * `stealth` - Boolean indicating whether to perform a stealth scan.
/// * `detect_service` - Boolean indicating whether to perform service detection.
///
/// # Returns
///
/// A `PortResult` struct indicating the status of the port.
fn scan_port(ip: IpAddr, port: u16, stealth: bool, detect_service: bool) -> PortResult {
    if stealth {
        // For stealth scanning, attempt to connect without completing the handshake.
        match TcpStream::connect((ip, port)) {
            Ok(mut stream) => {
                // Connection succeeded, so the port is considered open or filtered.
                if detect_service {
                    let mut response = String::new();
                    if let Err(_) = stream.read_to_string(&mut response) {
                        println!("Failed to detect service on port {}", port);
                    } else {
                        println!("Service on port {}: {}", port, response.trim());
                        let service = identify_service(port, stream); // Detect service
                        return PortResult { port, status: "open or filtered".to_string(), service };
                    }
                } else {
                    println!("Port {} is open or filtered", port);
                }
                PortResult { port, status: "open or filtered".to_string(), service: None }
            }
            Err(_) => {
                // Connection failed, so the port is considered closed.
                println!("Port {} is closed", port);
                PortResult { port, status: "closed".to_string(), service: None }
            }
        }
    } else {
        // For regular scanning, perform a standard TCP connect scan.
        match TcpStream::connect((ip, port)) {
            Ok(_) => {
                // Connection succeeded, so the port is considered open.
                println!("Port {} is open", port);
                PortResult { port, status: "open".to_string(), service: None }
            }
            Err(_) => {
                // Connection failed, so the port is considered closed.
                println!("Port {} is closed", port);
                PortResult { port, status: "closed".to_string(), service: None }
            }
        }
    }
}

/// Function to identify the service running on a port.
///
/// # Arguments
///
/// * `port` - The port number.
/// * `stream` - The TcpStream connected to the port.
///
/// # Returns
///
/// An `Option<String>` containing the detected service name, or None if no service is detected.
fn identify_service(port: u16, stream: TcpStream) -> Option<String> {
    match port {
        80 | 443 => identify_web_service(stream),
        _ => None,
    }
}

/// Function to identify the web service running on HTTP or HTTPS ports.
///
/// # Arguments
///
/// * `stream` - The TcpStream connected to the port.
///
/// # Returns
///
/// An `Option<String>` containing the detected web server name and version, or None if no web server is detected.
fn identify_web_service(mut stream: TcpStream) -> Option<String> {
    use std::io::Write;

    let request = b"GET / HTTP/1.0\r\n\r\n";

    if let Err(_) = stream.write_all(request) {
        return None;
    }

    let mut response = String::new();
    if let Err(_) = stream.read_to_string(&mut response) {
        return None;
    }

    if response.contains("HTTP/") {
        if response.contains("Server: nginx") {
            return Some("Nginx".to_string());
        } else if response.contains("Server: Apache") {
            return Some("Apache".to_string());
        }
    }

    None
}

fn main() {
    let matches = App::new("Port Scanner")
        .version("1.0")
        .author("Author: Dropa")
        .about("A simple port Scanner that helps me train Rust :3")
        .arg(
            Arg::with_name("ip")
                .long("ip")
                .value_name("IP_ADDRESS")
                .required(true)
                .help("Target IP address to scan"),
        )
        .arg(
            Arg::with_name("ports")
                .long("ports")
                .value_name("PORTS")
                .help("Only one port to scan or comma-separated list of ports to scan (e.g., 80,443)"),
        )
        .arg(
            Arg::with_name("start-port")
                .long("start-port")
                .value_name("START_PORT")
                .default_value("1")
                .help("Starting port for the scan"),
        )
        .arg(
            Arg::with_name("end-port")
                .long("end-port")
                .value_name("END_PORT")
                .default_value("1024")
                .help("Ending port for the scan"),
        )
        .arg(
            Arg::with_name("stealth")
                .long("stealth")
                .help("Perform a stealth scan"),
        )
        .arg(
            Arg::with_name("service")
                .long("service")
                .help("Perform service detection"),
        )
        .get_matches();

    // Retrieve command line arguments.
    let target_ip = matches
        .value_of("ip")
        .expect("The IP you want to scan needs is required!")
        .parse()
        .expect("Invalid IP address");

    let start_port: u16 = matches
        .value_of("start-port")
        .expect("")
        .parse()
        .expect("Invalid start port (Minimum 0 !)");

    let end_port: u16 = matches
        .value_of("end-port")
        .expect("")
        .parse()
        .expect("Invalid end port (max 65535)");

    let stealth = matches.is_present("stealth");
    let detect_service = matches.is_present("service");

    let results: Vec<PortResult>; // Vector to store scan results.

    // Perform scan based on user input and options.
    if let Some(ports_str) = matches.value_of("ports") {
        let ports: Vec<u16> = ports_str
            .split(',')
            .map(|p| p.parse().expect("Invalid port number (1 to 65535)"))
            .collect();

            results = ports
            .iter()
            .map(|&port| scan_port(target_ip, port, stealth, detect_service)) // Set detect_service to true
            .collect();
    } else {
        results = (start_port..=end_port)
            .map(|port| scan_port(target_ip, port, stealth, detect_service))
            .collect();
    }

    // Serialize scan results to JSON.
    let json_results = serde_json::to_string_pretty(&results).expect("Failed to serialize results");

    // Write JSON results to a file.
    let filename = format!("port_scan_results.json");
    std::fs::write(&filename, json_results).expect("Failed to write JSON file");

    println!("Results written to file: {}", filename);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_port_open() {
        // Start a dummy TCP server on a random port
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();

        // Spawn a thread to accept connections
        thread::spawn(move || {
            for stream in listener.incoming() {
                let _ = stream.unwrap();
            }
        });

        let result = scan_port(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port, false, false);

        assert_eq!(result.status, "open");
        assert_eq!(result.port, port);
        assert!(result.service.is_none());
    }

    #[test]
    fn test_scan_port_closed() {
        let result = scan_port(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9999, false, false);

        assert_eq!(result.status, "closed");
        assert_eq!(result.port, 9999);
        assert!(result.service.is_none());
    }
}