use clap::Parser;
use log::info;
use log4rs;

mod server;
mod client;
mod event;

/// Simple program to greet a person
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// execution mode - server or client
    #[arg(short, long, default_value_t = String::from("server"))]
    mode: String,

    /// address to listen 
    #[arg(short, long, default_value_t = String::from("0.0.0.0:7001"))]
    listen: String,

    /// address to connect
    #[arg(short, long, default_value_t = String::from("127.0.0.1:7001"))]
    remote: String,
 
    /// Number of connections 
    #[arg(short, long, default_value_t = 1)]
    conns: u32,

    /// Payload length
    #[arg(short, long, default_value_t = 1024)]
    size: u32,
}

fn main() {
    log4rs::init_file("log.yaml", Default::default()).unwrap();

    let args = Args::parse();

    info!("starting args: {:?}", args);

    if args.mode == "server" {
        server::run(args);
    }
    else if args.mode == "client" {
        client::run(args);
    }
    else {
        println!("server or client mode is supported");
    }
}