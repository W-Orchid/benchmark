use std::{ops::Deref, sync::Arc};

use clap::Parser;
use tokio::sync::mpsc::{channel, Sender, Receiver};
use womscp_lib::womscp::ResponseError;

mod init;
mod connections;
mod results;


#[tokio::main]
async fn main() {
    let cli = init::Cli::parse();

    let (sendr, mut recvr) 
        :(Sender<results::RequestBenchmark>, Receiver<results::RequestBenchmark>) 
          = channel(32);

    let mut results = results::Results::new(cli.number);

    let cli_ptr = Arc::from(cli);
    let local_cli_ptr = Arc::clone(&cli_ptr);

    connections::dispatcher(cli_ptr, sendr);

    while let Some(benchmark) = recvr.recv().await {
        if local_cli_ptr.verbose {
            println!("---Benchmark #{}---", benchmark.id);
            println!("Request: {:#?}", benchmark.request);
            println!("Response: {:#?}", benchmark.response);
            println!("Elapsed: {:#?}", benchmark.elapsed);
            println!("-------------------");
        }

        results.update(benchmark);
    }

    todo!("print results");
}
