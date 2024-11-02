use std::time::Instant;

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use tokio::sync::mpsc::Sender;

use womscp_lib::womscp::{Request, RequestFlags, ResponseError, WOMSCP_REQ_LEN, WOMSCP_VERSION};

use super::*;


pub fn dispatcher
(cli_ptr :Arc<init::Cli>, sendr :Sender<results::RequestBenchmark>) 
{
    let failure_point = cli_ptr.number * 
        (cli_ptr.failure / 100) as u32;

    let cli_loop_ptr :Arc<init::Cli> = Arc::clone(&cli_ptr);


    for i in 0..cli_loop_ptr.deref().number {
        let local_ptr :Arc<init::Cli> = Arc::clone(&cli_ptr);
        let local_sendr = sendr.clone();

        let req = Request {
            version: if i < failure_point {
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
                let benchmark = results::RequestBenchmark {
                    id: i,
                    elapsed: timer.elapsed(),
                    request: req,
                    response: Err(ResponseError::Tcp)
                };

                let _ = local_sendr.send(benchmark).await;
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
    let req_buf :[u8; WOMSCP_REQ_LEN] = request.try_into().unwrap();

    stream.write_all(&req_buf).await.unwrap();

    let mut res :[u8; 1] = [0];
    stream.read(&mut res).await.unwrap();

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
