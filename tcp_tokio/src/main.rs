use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

fn main() -> Result<(), Box<std::error::Error>> {
    let addr = "127.0.0.1:8888".parse()?;

    let socket = TcpListener::bind(&addr)?;
    println!("Listening on: {}", addr);

    let done = socket
        .incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            let (reader, writer) = socket.split();
            tokio::spawn(
                io::lines(std::io::BufReader::new(reader))
                    .filter(|line| line.starts_with("PING"))
                    .fold(
                        writer,
                        |writer, _| {
                            io::write_all(writer, b"+PONG\r\n").map(|(w, _)| w)
                        })
                    .then(|_| Ok(()))
            )
        });

    tokio::run(done);
    Ok(())
}
