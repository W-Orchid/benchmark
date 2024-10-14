use std::net::TcpStream;
use std::io::Write;

fn main() {

    for _ in 1..10 {
        let buf = [1, 0, 0x0d, 5, 3, 0, 0, 0, 0x7b];

        let mut stream = TcpStream::connect("127.0.0.1:8000").unwrap();
        stream.write(&buf).expect("Failed to write to TCP stream!");
    }
}
