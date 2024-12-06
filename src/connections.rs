use std::time::Instant;

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use tokio::sync::mpsc::Sender;

use womscp_lib::womscp::{Request, RequestFlags, ResponseError, WOMSCP_REQ_LEN, WOMSCP_VERSION};

use super::*;


pub async fn dispatcher
(cli_ptr :Arc<init::Cli>, sendr :Sender<results::RequestBenchmark>) 
{
    for i in 
        0..((cli_ptr.deref().number as f32 / cli_ptr.deref().concurrent as f32) as u32)
        {
            handle_concurrent_group(i, Arc::clone(&cli_ptr), &sendr).await;
        }
}


async fn handle_concurrent_group(group_id :u32, cli_ptr :Arc<init::Cli>, sendr :&Sender<results::RequestBenchmark>) {
    let failure_point = (cli_ptr.number as f32) * 
        (cli_ptr.failure as f32 / 100.0);

    let mut handles :Vec<tokio::task::JoinHandle<()>> = vec!();

    for j in 0..cli_ptr.deref().concurrent {
        let local_ptr :Arc<init::Cli> = Arc::clone(&cli_ptr);
        let local_sendr = sendr.clone();

        let handle = tokio::spawn(async move {
            let req = Request {
                version: if (group_id as f32) * (local_ptr.deref().concurrent as f32) + (j as f32) < failure_point {
                    2 as u8
                } else {
                    WOMSCP_VERSION
                },
                m_id: 0 as u16,
                s_id: 0 as u8,
                sensor_type: 1 as u8,
                data: rand::random::<u32>(),
                flags: RequestFlags::Dummy as u8
            };

            let timer = Instant::now();

            let cli = local_ptr.deref();
            let stream = TcpStream::connect(&cli.address).await;

            if stream.is_err() {
                eprintln!("TCP error: {:#?}", stream.unwrap_err());

                let benchmark = results::RequestBenchmark {
                    id: group_id,
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
                id: group_id,
                elapsed: timer.elapsed(),
                request: req,
                response: res
            };

            local_sendr.send(benchmark).await.unwrap();
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }
}

async fn send_request
(stream :&mut TcpStream, request :&Request) -> Result<(), ResponseError> 
{
    let req_buf :[u8; WOMSCP_REQ_LEN] = request.try_into()?;
    let req_test :Request = Request::try_from(&req_buf)?;

    assert_eq!(request, &req_test);

    if let Err(e) = stream.write_all(&req_buf).await {
        eprintln!("TCP write error: {}", e);
        return Err(ResponseError::Tcp);
    }

    let mut res :[u8; 1] = [0];
    if let Err(e) = stream.read(&mut res).await {
        eprintln!("TCP read error: {}", e);
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
