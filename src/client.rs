use std::sync::mpsc::Receiver;
use tokio::runtime::Builder;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use bytes::{BytesMut, BufMut};
use anyhow;
use log::{debug, info, error};

pub fn run(args: super::Args) {
    let runtime = Builder::new_multi_thread()
        .enable_io()
        .thread_name("run-tcp")
        .build()
        .unwrap();

    let result = runtime.block_on(run_tcp(&args));
    if let Err(e) = result {
        error!("error: {:?}", e);
    }
}

async fn run_tcp(args: &super::Args) -> Result<(), anyhow::Error> {

    let mut num_connection = 0;
    let mut handles = vec![];

    info!("connecting {} clients", args.conns);

    while num_connection < args.conns {

        let stream = TcpStream::connect(&args.remote).await?;
        info!("connected to {}", args.remote);

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
    let mut echo_count = 0;
    let run = true;

    buf.put_bytes(1, echo_size as usize);

    let peer = stream.peer_addr().unwrap();

    while run {
        stream.write_buf(&mut buf).await?;
        buf.clear();

        let read = stream.read_buf(&mut buf).await;
        match read {
            Ok(n) => {
                if n == 0 {
                    error!("remote disconnected. peer:{}", peer);
                    break;
                }
                else {
                    debug!("echo size: {}", n);
                }
            }
            Err(e) => {
                error!("recv error:{}, peer:{}", e, peer);
                return Err(e.into());
            }
        } 

        echo_count += 1;
        debug!("echo: {}", echo_count);
    }

    Ok(())
}
