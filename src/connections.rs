use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use tokio::sync::mpsc::Sender;

use womscp_lib::womscp::{Request, RequestFlags, ResponseError, WOMSCP_REQ_LEN, WOMSCP_VERSION};

use super::*;

pub fn dispatcher
(cli :init::Cli, sendr :Sender<Result<(), ResponseError>>) 
{
    let failure_point = cli.number * (cli.failure / 100) as u32;
    let cli_ptr :Arc<init::Cli> = Arc::from(cli);
    let cli_loop_ptr :Arc<init::Cli> = Arc::clone(&cli_ptr);


    for i in 0..cli_loop_ptr.deref().number {
        let local_ptr :Arc<init::Cli> = Arc::clone(&cli_ptr);
        let local_sendr = sendr.clone();

        tokio::spawn(async move {
            let cli = local_ptr.deref();
            let stream = TcpStream::connect(&cli.address).await;

            if stream.is_err() {
                let _ = local_sendr.send(Err(ResponseError::Tcp)).await;
                return;
            }

            let res = connections::send_request(
                i,
                &mut stream.unwrap(), 
                i < failure_point,
                cli.verbose
                ).await;

            local_sendr.send(res).await.unwrap();
        });
    }


}

pub async fn send_request
(request_id :u32, stream :&mut TcpStream, failure :bool, verbose :bool) -> Result<(), ResponseError> 
{
    let req = Request {
        version: if failure {
            0
        } else { 
            WOMSCP_VERSION 
        },
        m_id: 0,
        s_id: 0,
        sensor_type: 1,
        data: 0,
        flags: RequestFlags::Dummy as u8
    };

    if verbose {
        println!("---REQUEST #{}---\n{:#?}", request_id, req);
    }

    let req_buf :[u8; WOMSCP_REQ_LEN] = req.try_into().unwrap();

    stream.write_all(&req_buf).await.unwrap();

    let mut res :[u8; 1] = [0];
    stream.read(&mut res).await.unwrap();

    if verbose {
        println!("---RESPONSE #{}---\n{:#?}", request_id, res);
    }

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
