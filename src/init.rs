use clap::{Parser, Subcommand};

const DEFAULT_ADDR :&str = "127.0.0.1:3000";

#[derive(Parser)]
#[command(name = "womscp-benchmark")]
#[command(version = "1.0")]
#[command(about = "Runs a benchmark on the server that handles the WOMSCP.", long_about = None)]
pub struct Cli {
    /// Optional address to send to. Default is 127.0.0.1:3000
    #[arg(short, long)]
    pub address :Option<String>,

    /// Failure rate
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..100))] 
    pub failure :u8,

    /// Verbose (print out requests and responses)
    #[arg(short, long)]
    pub verbose :bool,
}
