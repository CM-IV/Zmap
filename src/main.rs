use std::net::TcpStream;

use argh::FromArgs;
use color_eyre::eyre::Result;
use ansi_term::Colour;
use rayon::prelude::*;


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

    let open_ports: Vec<u16> = (1u16..65535u16)
        .into_par_iter()
        .filter_map(|port| {
            let address = format!("{}:{}", &ip, port);
            if TcpStream::connect(address.as_str()).is_ok() {
                Some(port)
            } else {
                None
            }
        })
        .collect();
    
    if open_ports.is_empty() {
        println!("{}", Colour::Green.paint("No open ports found"));
    } else {
        for port in open_ports {
            println!("Port {} is open", Colour::Red.paint(port.to_string()));
        }
    }

    println!("{}", Colour::Green.paint("Done!"));

    Ok(())
}
