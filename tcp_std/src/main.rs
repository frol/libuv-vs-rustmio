use std::net::TcpListener;
use std::io::{BufRead, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();

    for stream in listener.incoming() {
        std::thread::spawn(move || {
            match stream {
                Ok(mut stream) => {
                    let mut reader = std::io::BufReader::new(stream.try_clone().unwrap());
                    let mut buffer = Vec::new();
                    while let Ok(read_bytes) = reader.read_until(b'\n', &mut buffer) {
                        if read_bytes == 0 {
                            break;
                        }
                        let response = b"+PONG\r\n";
                        stream.write(response).expect("Response failed");
                    }
                }
                Err(e) => {
                    println!("Unable to connect: {}", e);
                }
            }
        });
    }
}
