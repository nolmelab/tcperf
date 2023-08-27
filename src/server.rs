use crate::event::Event;
use anyhow;
use bytes::BytesMut;
use ratatui::{backend::CrosstermBackend, widgets::Paragraph, Terminal};
use std::net::SocketAddr;
use std::sync::mpsc::{self, Receiver, Sender};
use std::{io::stdout, thread, time::Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Builder;
use log::{debug, info, warn, error};

pub fn run(args: super::Args) {
    let runtime = Builder::new_multi_thread()
        .enable_io()
        .thread_name("run-tcp")
        .build()
        .unwrap();

    // 소유권을 자세히 잘 정리하면 안정성에 큰 도움이 된다. 그것이 러스트다.
    let args2 = args.clone();

    let _result = runtime.block_on(run_tcp(&args));
}

async fn run_tcp(args: &super::Args) -> Result<(), anyhow::Error> {
    // listener를 만들고 accept를 하면 task로 각 클라 연결에 대해 echo 처리
    let listener = TcpListener::bind(&args.listen).await?;
    info!("listening on {}", args.listen);

    let running = true;

    while running {
        let result = listener.accept().await;
        match result {
            Ok((stream, remote_addr)) => {
                info!("accepted:{}", remote_addr);
                let echo_size = args.size.clone();

                tokio::spawn(async move {
                    let _ = run_stream(echo_size, stream).await;
                });
            }
            Err(e) => {
                error!("accept error:{:?}", e);
            }
        }
    }

    Ok(())
}

// write_buf()가 mut 참조를 필요로 한다. stream을 mut로 전달한다.
async fn run_stream(
    echo_size: u32,
    mut stream: TcpStream,
) -> Result<(), anyhow::Error> {
    let peer = stream.peer_addr().unwrap();
    let mut buf = BytesMut::with_capacity(echo_size as usize);
    let run = true;
    let mut echo_count = 0;

    while run {
        let read = stream.read_buf(&mut buf).await;
        match read {
            Ok(n) => {
                if n == 0 {
                    error!("remote disconnected. peer:{}", peer);
                    break;
                }
            }
            Err(e) => {
                error!("recv error:{}, peer:{}", e, peer);
                return Err(e.into());
            }
        }

        stream.write_buf(&mut buf).await?;
        buf.clear();

        echo_count += 1;
        debug!("echo: {}", echo_count);
    }

    Ok(())
}
