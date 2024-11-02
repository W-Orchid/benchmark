use std::time::Instant;

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use tokio::sync::mpsc::Sender;

use womscp_lib::womscp::{Request, RequestFlags, ResponseError, WOMSCP_REQ_LEN, WOMSCP_VERSION};

use super::*;


pub fn dispatcher
(cli_ptr :Arc<init::Cli>, sendr :Sender<results::RequestBenchmark>) 
{
    let failure_point = (cli_ptr.number as f32) * 
        (cli_ptr.failure as f32 / 100.0);

    let cli_loop_ptr :Arc<init::Cli> = Arc::clone(&cli_ptr);


    for i in 0..cli_loop_ptr.deref().number {
        let local_ptr :Arc<init::Cli> = Arc::clone(&cli_ptr);
        let local_sendr = sendr.clone();

        let req = Request {
            version: if (i as f32) < failure_point {
                0
            } else {
                WOMSCP_VERSION
            },
            m_id: 0,
            s_id: 0,
            sensor_type: 1,
            data: 123,
            flags: RequestFlags::Dummy as u8
        };

        let timer = Instant::now();

        tokio::spawn(async move {
            let cli = local_ptr.deref();
            let stream = TcpStream::connect(&cli.address).await;

            if stream.is_err() {
                eprintln!("TCP error: {:#?}", stream.unwrap_err());

                let benchmark = results::RequestBenchmark {
                    id: i,
                    elapsed: timer.elapsed(),
                    request: req,
                    response: Err(ResponseError::Tcp)
                };

                local_sendr.send(benchmark).await.unwrap();
                return;
            }

            let res = connections::send_request(
                &mut stream.unwrap(), 
                &req
                ).await;

            let benchmark = results::RequestBenchmark {
                id: i,
                elapsed: timer.elapsed(),
                request: req,
                response: res
            };

            local_sendr.send(benchmark).await.unwrap();
        });
    }


}

pub async fn send_request
(stream :&mut TcpStream, request :&Request) -> Result<(), ResponseError> 
{
    let req_buf :[u8; WOMSCP_REQ_LEN] = request.try_into()?;

    if let Err(e) = stream.write_all(&req_buf).await {
        eprintln!("TCP write error: {}", e);
        return Err(ResponseError::Tcp);
    }

    let mut res :[u8; 1] = [0];
    if let Err(e) = stream.read(&mut res).await {
        eprintln!("TCP write error: {}", e);
        return Err(ResponseError::Tcp);
    };

    match res {
        [0] => Ok(()),
        [1] => Err(ResponseError::NotReady),
        [2] => Err(ResponseError::Version),
        [3] => Err(ResponseError::Unrecognised),
        [4] => Err(ResponseError::Tcp),
        [5] => Err(ResponseError::Database),
        _   => { panic!("Unrecognised response code: {}", res[0]); }
    }
}
