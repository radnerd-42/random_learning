use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::io::{self, Write};
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use ipnet::IpNet;
use rand::seq::SliceRandom;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ScanResult {
    ip: String,
    port: u16,
    status: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Cybersecurity Internal Scanner ---");

    // 1. Collect IP Addresses
    let ips = collect_input("Enter IP range (CIDR/Full) or path to .txt file: ", parse_ips)?;
    
    // 2. Collect Ports
    let ports = collect_input("Enter Ports (comma-separated) or path to .txt file: ", parse_ports)?;

    // 3. Randomize selection
    let mut rng = rand::thread_rng();
    let mut targets: Vec<(IpAddr, u16)> = Vec::new();
    for ip in &ips {
        for port in &ports {
            targets.push((*ip, *port));
        }
    }
    targets.shuffle(&mut rng);

    println!("\nStarting scan on {} targets...\n", targets.len());

    let mut results = Vec::new();

    // 4. Perform Scan
    for (ip, port) in targets {
        let addr = SocketAddr::new(ip, port);
        let status = match timeout(Duration::from_millis(800), TcpStream::connect(addr)).await {
            Ok(Ok(_)) => "open",
            _ => "closed",
        };

        let res = ScanResult {
            ip: ip.to_string(),
            port,
            status: status.to_string(),
        };
        results.push(res);

        // Immediate terminal feedback
        println!("IP: {}\n  Port {}: {}", ip, port, status);
    }

    // 5. JSON Export
    print!("\nExport results to JSON? (y/n): ");
    io::stdout().flush()?;
    let mut export = String::new();
    io::stdin().read_line(&mut export)?;

    if export.trim().to_lowercase() == "y" {
        let json = serde_json::to_string_pretty(&results)?;
        std::fs::write("scan_results.json", json)?;
        println!("Results saved to scan_results.json");
    }

    Ok(())
}

fn collect_input<T>(prompt: &str, parser: fn(&str) -> Vec<T>) -> io::Result<Vec<T>> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if input.ends_with(".txt") {
        let content = read_to_string(input)?;
        Ok(parser(&content))
    } else {
        Ok(parser(input))
    }
}

fn parse_ips(input: &str) -> Vec<IpAddr> {
    input.split(|c| c == ',' || c == '\n' || c == ' ')
        .filter_map(|s| {
            if let Ok(net) = s.parse::<IpNet>() {
                Some(net.hosts().collect::<Vec<IpAddr>>())
            } else if let Ok(ip) = s.parse::<IpAddr>() {
                Some(vec![ip])
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

fn parse_ports(input: &str) -> Vec<u16> {
    input.split(|c| c == ',' || c == '\n' || c == ' ')
        .filter_map(|s| s.trim().parse::<u16>().ok())
        .collect()
}