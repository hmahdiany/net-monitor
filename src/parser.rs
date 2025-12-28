use std::{fmt, fs};
use crate::network;
use colored::Colorize;

pub struct Connection {
    pub local_address: String,
    pub remote_address: String,
    pub connection_state: String,
    pub protocol: String,
    pub pid: String,
    pub program_name: String,
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Pad the state string MANUALLY to 15 chars before coloring it
        let padded_state = format!("{: <15}", self.connection_state);
        
        // Color the already-padded string
        let colored_state = match self.connection_state.as_str() {
            "LISTEN" => padded_state.yellow(),
            "ESTABLISHED" => padded_state.green(),
            "TIME_WAIT" => padded_state.blue(),
            "CLOSE_WAIT" => padded_state.magenta(),
            "UDP_LISTEN" => padded_state.cyan(),
            _ => padded_state.normal(),
        };

        write!(f, "{:<5} | {:<15} | {:<25} | {:<25} | {:<15} | {:<8}",
            self.protocol,
            colored_state,
            self.local_address,
            self.remote_address,
            self.program_name,
            self.pid,
        )
    }
}

pub fn parse_connections(contents: String, proto: &str) -> Vec<Connection> {
    let data_lines = contents.lines().skip(1);

    let connections: Vec<Connection> = data_lines
        .filter_map(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();

            let inode = fields.get(9).copied()?;

            let pid_string = network::find_pid_from_inode(&inode)?;

            let program_name = fs::read_to_string(format!("/proc/{pid_string}/comm"))
                .ok()?
                .trim()
                .to_string();

            Some(Connection {
                local_address: network::decode_address(fields.get(1)?.to_string()),
                remote_address: network::decode_address(fields.get(2)?.to_string()),
                connection_state: network::decode_stage(fields.get(3)?).to_string(),
                protocol: String::from(proto),
                program_name,
                pid: pid_string,
            })
        })
        .collect();

    return connections;
}