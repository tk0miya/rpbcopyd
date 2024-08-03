use std::process::Command;

use axum::{routing::get, Router};
use clap::Parser;
use tokio::net::TcpListener;

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    daemon: bool,

    #[arg(short = 'H', long, default_value = "localhost")]
    host: String,

    #[arg(short, long, default_value_t = 12345)]
    port: u16,
}

async fn pbpaste() -> Vec<u8> {
    let output = Command::new("/usr/bin/pbpaste").output();
    output.unwrap().stdout
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("Listening on {}:{}", args.host, args.port);

    let addr = [args.host, args.port.to_string()].join(":");
    let listener = TcpListener::bind(addr).await.unwrap();

    let app = Router::new().route("/", get(pbpaste));
    axum::serve(listener, app).await.unwrap();
}
