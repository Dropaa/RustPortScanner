use std::net::{IpAddr, TcpStream};
use clap::{App, Arg};

fn scan_port(ip: IpAddr, port: u16) {
    match TcpStream::connect((ip, port)) { //Match = sorte de try and catch en python. Si le flux tcp arrive à se connecter au port renvoi Ok sinon Err.
        Ok(_) => println!("Port {} is open", port), // Ici l'underscore est un "plcaeholder" vu que je ne peux pas faire Ok(). Je pourrais catch la réponse du Stream et print le Stream également.
        Err(_) => println!("Port {} close", port),
    }
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
                .help("Onnly one port to scan or comma-separated list of ports to scan (e.g., 80,443)"),
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
        .get_matches();

    // On récupère les arguments depuis la ligne de commande
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

    // Si l'argument --ports est utilisé, on ne va scanner que ceux-la et ne pas passer par une range de ports
    if let Some(ports_str) = matches.value_of("ports") { //Si il y a bien l'argument --ports, matches.value_of("ports") retournera "Some" sinon "None".
        let ports: Vec<u16> = ports_str // Je vais stocker tous les ports dans un vecteur (tableau like mais en mieux :3)
            .split(',') // Je sépare les différents ports séparé par une virgule avec la fonction split
            .map(|p| p.parse().expect("Invalid port number (1 to 65535)")) // Prends chaque item dans la liste et convertis le port initialement en str en u16
            .collect(); // On collecte les résultats dans un vecteur

        // Scan specified ports
        for &port in &ports {
            scan_port(target_ip, port);
        }
    } else {
        // Scan the default range from start_port to end_port
        for port in start_port..=end_port {
            scan_port(target_ip, port);
        }
    }
}