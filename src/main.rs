use std::net::TcpStream;

use argh::FromArgs;
use color_eyre::eyre::Result;
use ansi_term::Colour;


#[derive(FromArgs)]
#[argh(description = r#"
Zmap, a simple nmap alternative written in Rust.
Scans for opens ports from port 1 - 65535.

By CM-IV <chuck@civdev.xyz>
"#)]
struct Args {
    #[argh(positional, description = "IP address to scan")]
    ip: String
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();
    let ip = args.ip;

    println!("{}", Colour::Yellow.paint("Scanning..."));
    
    let mut open_ports = Vec::new();
    for port in 1..65535 {
        let address = format!("{}:{}", ip, port);
        if TcpStream::connect(address).is_ok() {
            println!("Port {} is open", Colour::Red.paint(port.to_string()));
            open_ports.push(port);
        }
    }
    if open_ports.is_empty() {
        println!("{}", Colour::Green.paint("No open ports found"));
    }

    println!("{}", Colour::Green.paint("Done!"));

    Ok(())
}
