use std::net::{IpAddr, TcpStream};
use clap::{App, Arg};
use serde::Serialize;


#[derive(Serialize)]
struct PortResult {
    port: u16,
    status: String,
}

fn scan_port(ip: IpAddr, port: u16) -> PortResult {
    match TcpStream::connect((ip, port)) { //Match = sorte de try and catch en python. Si le flux tcp arrive à se connecter au port renvoi Ok sinon Err.
        Ok(_) => { // Ici l'underscore est un "plcaeholder" vu que je ne peux pas faire Ok(). Je pourrais catch la réponse du Stream et print le Stream également.
            println!("Port {} is open", port);
            PortResult { port, status: "open".to_string() }
        }
        Err(_) => {
            println!("Port {} closed", port);
            PortResult { port, status: "closed".to_string() }
        }
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

    let results: Vec<PortResult>; // On va créer ici un vecteur qui à la forme de la structure de PortResult --> port, status

    // Si l'argument --ports est utilisé, on ne va scanner que ceux-la et ne pas passer par une range de ports
    if let Some(ports_str) = matches.value_of("ports") { //Si il y a bien l'argument --ports, matches.value_of("ports") retournera "Some" sinon "None".
        let ports: Vec<u16> = ports_str // Je vais stocker tous les ports dans un vecteur (tableau like mais en mieux :3)
            .split(',') // Je sépare les différents ports séparé par une virgule avec la fonction split
            .map(|p| p.parse().expect("Invalid port number (1 to 65535)")) // Prends chaque item dans la liste et convertis le port initialement en str en u16
            .collect(); // On collecte les résultats dans un vecteur

        results = ports
            .iter()
            .map(|&port| scan_port(target_ip, port))
            .collect();

    } else {
        // Scan la range d'IP commencant par start_port et finissant pas end_port
        results = (start_port..=end_port)
            .map(|port| scan_port(target_ip, port))
            .collect();
    }

    // On sérialise les résultats dans un fichier JSON
    let json_results = serde_json::to_string_pretty(&results).expect("Failed to serialize results");

    // On écris le JSON dans un fichier
    let filename = format!("port_scan_results.json");
    std::fs::write(&filename, json_results).expect("Failed to write JSON file");

    println!("Results written to file: {}", filename);
}
