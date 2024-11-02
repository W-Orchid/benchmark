use std::{ops::Deref, sync::Arc};

use clap::Parser;
use tokio::sync::mpsc::{channel, Sender, Receiver};
use womscp_lib::womscp::ResponseError;

mod init;
mod connections;


#[tokio::main]
async fn main() {
    let cli = init::Cli::parse();

    type ChannelType = Result<(), ResponseError>;

    let (sendr, mut recvr) 
        :(Sender<ChannelType>, Receiver<ChannelType>) 
          = channel(32);

    connections::dispatcher(cli, sendr);

    while let Some(res) = recvr.recv().await {
        match res {
            Ok(inner) => { dbg!(inner); },
            Err(e) => { dbg!(e); }
        };

        todo!("implements some saving of results");
    }


}
