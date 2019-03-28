use tokio::codec::Framed;
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::codec::{Encoder, Decoder};
use tokio::runtime::current_thread::Runtime;

use bytes::{BufMut, BytesMut};

struct C;

impl Encoder for C {
    type Item = u64;
    type Error = std::io::Error;
    fn encode(
        &mut self,
        item: Self::Item,
        dst: &mut BytesMut
    ) -> Result<(), Self::Error> {
        const PONG: &[u8] = b"+PONG\r\n";
        for _ in 0..item {
            dst.reserve(PONG.len());
            dst.put(PONG.as_ref());
        }
        Ok(())
    }
}

impl Decoder for C {
    type Item = u64;
    type Error = std::io::Error;

    fn decode(
        &mut self,
        src: &mut BytesMut
    ) -> Result<Option<Self::Item>, Self::Error> {
        let count = bytecount::count(&src, b'\n') as u64;
        if count > 0 {
            src.clear();
            Ok(Some(count))
        } else {
            Ok(None)
        }
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let addr = "127.0.0.1:8888".parse()?;

    let socket = TcpListener::bind(&addr)?;
    println!("Listening on: {}", addr);

    let done = socket
        .incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            tokio::spawn({
                let framed = Framed::new(socket, C);
                let (writer, reader) = framed.split();
                reader.forward(writer).map_err(|_| ()).map(|_| ())
            })
        });

    let mut rt = Runtime::new().unwrap();
    rt.block_on(done).unwrap();
    Ok(())
}
