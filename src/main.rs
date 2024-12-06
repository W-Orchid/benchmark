use std::{ops::Deref, sync::Arc};

use clap::Parser;
use tokio::sync::mpsc::{channel, Sender, Receiver};
use tokio::time::Instant;
use womscp_lib::womscp::ResponseError;

mod init;
mod connections;
mod results;


#[tokio::main]
async fn main() {
    let mut cli = init::Cli::parse();

    if cli.number < (cli.concurrent as u32) {
        cli.concurrent = cli.number as u16;
    }

    let (sendr, mut recvr) 
        :(Sender<results::RequestBenchmark>, Receiver<results::RequestBenchmark>) 
          = channel(cli.concurrent as usize);

    let mut results = results::Results::new(cli.number, cli.concurrent, cli.failure);

    let cli_ptr = Arc::from(cli);
    let local_cli_ptr = Arc::clone(&cli_ptr);

    let timer = Instant::now();
    tokio::task::spawn(connections::dispatcher(cli_ptr, sendr));

    let mut requests_handled = 0;

    while let Some(benchmark) = recvr.recv().await {
        if local_cli_ptr.verbose {
            println!("---Benchmark #{}---", benchmark.id);
            println!("Request: {:#?}", benchmark.request);
            println!("Response: {:#?}", benchmark.response);
            println!("Elapsed: {:#?}", benchmark.elapsed);
            println!("-------------------");
        }

        results.update(benchmark);
        requests_handled += 1;

        if requests_handled == local_cli_ptr.number {
            break;
        }
    }

    results.total_response_time = timer.elapsed();
    println!("{}", results);
}
