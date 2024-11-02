use clap::Parser;


#[derive(Parser)]
#[command(name = "womscp-benchmark")]
#[command(version = "1.0")]
#[command(about = "Runs a benchmark on the server that handles the WOMSCP.", long_about = None)]
pub struct Cli {
    /// Number of requests to make
    #[arg(short, long, default_value_t = 100_000)]
    pub number :u32,

    /// Address to send to
    #[arg(short, long, default_value_t = String::from("127.0.0.1:3000"))]
    pub address :String,

    /// Failure rate
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(0..100), default_value_t = 0)] 
    pub failure :u8,

    /// Verbose (print out requests and responses)
    #[arg(short, long)]
    pub verbose :bool,
}
