use std::sync::mpsc::{ self, Receiver, Sender };
use std::thread;
use tokio::runtime::Builder;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use bytes::{BytesMut, BufMut};
use anyhow;
use crate::event::Event;

pub fn run(args: super::Args) {
    let (tx, rx) = mpsc::channel::<Event>();

    let runtime = Builder::new_multi_thread()
        .enable_io()
        .thread_name("run-tcp")
        .build()
        .unwrap();

    let args2 = args.clone();

    // thread::spawn(move || {
      //   run_ui(&args2, rx);
    // });

    let _result = runtime.block_on(run_tcp(&args, tx));
}

async fn run_tcp(args: &super::Args, tx: Sender<Event>) -> Result<(), anyhow::Error> {

    let mut num_connection = 0;
    let mut handles = vec![];

    while num_connection < args.conns {

        let stream = TcpStream::connect(&args.listen).await?;
        let echo_size = args.size.clone();

        let handle = tokio::spawn(async move {
            let _ = run_stream(echo_size, stream).await;
        });

        handles.push(handle);
        num_connection += 1;
    }

    // wait for all tasks to complete
    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}

async fn run_stream(echo_size: u32, mut stream: TcpStream) -> Result<(), anyhow::Error> {
    let mut buf = BytesMut::with_capacity(echo_size as usize);
    let run = true;

    buf.put_bytes(1, echo_size as usize);

    while run {
        stream.write_buf(&mut buf).await?;
        buf.clear();
        stream.read_buf(&mut buf).await?;
    }

    Ok(())
}

fn run_ui(args: &super::Args, rx: Receiver<Event>) {

}