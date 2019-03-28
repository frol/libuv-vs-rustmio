#![allow(deprecated)]
use std::io;

use futures::{future, Future, BoxFuture};
use tokio_core::io::{Io, Codec, Framed, EasyBuf};
use tokio_proto::TcpServer;
use tokio_proto::pipeline::ServerProto;
use tokio_service::Service;

#[derive(Default)]
pub struct BytesCodec;

impl Codec for BytesCodec {
    type In = Vec<u8>;
    type Out = Vec<u8>;

    fn decode(&mut self, buf: &mut EasyBuf) -> Result<Option<Self::Out>, io::Error> {
        if let Some(i) = buf.as_slice().iter().position(|&b| b == b'\n') {
            // remove the line, including the '\n', from the buffer
            let full_line = buf.drain_to(i + 1);

            // strip the'`\n'
            let slice = &full_line.as_slice()[..i];

            Ok(Some(slice.to_owned()))
        } else {
            Ok(None)
        }
        //Ok(Some(buf.as_slice().to_owned()))
    }

    fn decode_eof(&mut self, buf: &mut EasyBuf) -> io::Result<Self::Out> {
        Ok(buf.as_slice().to_owned())
    }

    fn encode(&mut self, item: Self::In, into: &mut Vec<u8>) -> io::Result<()> {
        into.extend(item);
        Ok(())
    }
}

pub struct MyProto;

impl<T: Io + 'static> ServerProto<T> for MyProto {
    type Request = Vec<u8>;
    type Response = Vec<u8>;
    type Transport = Framed<T, BytesCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(BytesCodec))
    }
}

pub struct Ponger;

impl Service for Ponger {
    type Request = Vec<u8>;
    type Response = Vec<u8>;
    type Error = io::Error;
    type Future = BoxFuture<Vec<u8>, io::Error>;

    fn call(&self, _req: Vec<u8>) -> Self::Future {
        future::finished(b"+PONG\r\n".to_vec()).boxed()
    }
}

fn main() {
    let addr = "0.0.0.0:8888".parse().unwrap();
    TcpServer::new(MyProto, addr)
        .serve(|| Ok(Ponger));
}
