#![feature(async_await, await_macro, futures_api, slice_patterns, repeat_generic_slice)]

use std::io;

use futures::StreamExt;
use futures::io::{AsyncReadExt, AsyncWriteExt};
use futures::sink::SinkExt;
use futures::task::{SpawnExt};

use romio::TcpListener;

fn main() -> io::Result<()> {
    /* Thread pool
    let mut root_executor = futures::executor::ThreadPool::new().unwrap();
    let mut executor = root_executor.clone();
    root_executor.run(async {
    // */

    //* Single threaded executor
    let mut root_executor = futures::executor::LocalPool::new();
    let mut executor = root_executor.spawner();
    root_executor.run_until(async {
    // */
        let mut listener = TcpListener::bind(&"127.0.0.1:8888".parse().unwrap())?;
        let mut incoming = listener.incoming();

        while let Some(stream) = await!(incoming.next()) {
            //let (mut sender, mut receiver) = futures::channel::mpsc::channel::<Vec<u8>>(1000);
            //let (mut sender, mut receiver) = futures::channel::mpsc::unbounded::<Vec<u8>>();
            let (mut sender, mut receiver) = futures::channel::mpsc::unbounded::<usize>();
            let (mut reader, mut writer) = stream.unwrap().split();
            let mut executor2 = executor.clone();
            executor.spawn(async move {
                //while let Ok(_) = await!(writer.write_all(b"+PONG\r\n")) {
                //while let Some(msg) = await!(receiver.next()) {
                while let Some(number_of_commands) = await!(receiver.next()) {
                    let response = b"+PONG\r\n".repeat(number_of_commands);
                    await!(writer.write_all(&response)).unwrap();
                }
            }).unwrap();
            executor2.spawn(async move {
                let mut buffer = [0; 1024];
                while let Ok(read_bytes) = await!(reader.read(&mut buffer)) {
                    if read_bytes == 0 {
                        break;
                    }

                    let number_of_commands = buffer.iter().filter(|&&x| x == b'\n').count();
                    //let response = b"+PONG\r\n".repeat(number_of_commands);
                    //await!(sender.send(response)).unwrap();
                    await!(sender.send(number_of_commands)).unwrap();
                    //buffer = [0; 1024];
                }
            }).unwrap();
        }

        Ok(())
    })
}
