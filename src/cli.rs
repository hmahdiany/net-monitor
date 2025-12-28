use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum Protocol {
    Tcp,
    Udp,
    All,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// nt: A simple and fast network monitor
pub struct Args {
    /// The protocol to filter
    #[arg(short, long, value_enum, default_value_t = Protocol::All, ignore_case = true)]
    pub protocol: Protocol,

    /// Interval in seconds to refresh the output (optional)
    #[arg(short, long)]
    pub wait: Option<u64>,
}