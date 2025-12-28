
use core::time;
use std::{error::Error, thread};
use clap::Parser;
use net_monitor::parser;
use net_monitor::network;
use net_monitor::cli;

fn main() -> Result<(), Box<dyn Error>> {
    let args=  cli::Args::parse();

    match args.wait {
        Some(wait) => {
            loop {
                // Clear the screen and move cursor to top-left
                print!("\x1B[2J\x1B[1;1H");
                display_connections(&args)?;
                thread::sleep(time::Duration::from_secs(wait));
            }
        }
        None => {
            display_connections(&args)?;
        }
    }

    Ok(())
}

fn display_connections(args: &net_monitor::cli::Args) -> Result<(), Box<dyn Error>> {    
    println!("{:<5} | {:<15} | {:<25} | {:<25} | {:<15} | {:<8}", 
    "PROTO", "STATE", "LOCAL", "REMOTE", "PROGRAM", "PID");
    println!("-------------------------------------------------------------------------------------------------------");

    let mut all_connections: Vec<net_monitor::parser::Connection> = Vec::new();
    let mut fetch_tcp = false;
    let mut fetch_udp = false;

    match args.protocol {
        cli::Protocol::Tcp => fetch_tcp = true,
        cli::Protocol::Udp => fetch_udp = true,
        cli::Protocol::All => {
            fetch_tcp = true;
            fetch_udp = true;
        }
    }

    if fetch_tcp {
        let tcp_data = network::get_tcp_connections()?;
        let tcp_connections = parser::parse_connections(tcp_data, "TCP");
        all_connections.extend(tcp_connections);
    }

    if fetch_udp {
        let udp_data = network::get_udp_connections()?;
        let udp_connections = parser::parse_connections(udp_data, "UDP");
        all_connections.extend(udp_connections);
    }
    
    all_connections.sort_by_key(|c| c.program_name.to_lowercase());

    for connection in all_connections {
        println!("{}", connection);
    }

    Ok(())
}