use std::io::Write;
use std::process::{Command, Stdio};

use axum::{
    extract::Request,
    routing::{get, post},
    Router,
};
use clap::Parser;
use daemonize::Daemonize;
use futures_util::StreamExt;
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

async fn pbcopy(request: Request) {
    let mut stream = request.into_body().into_data_stream();
    let pbcopy = Command::new("/usr/bin/pbcopy")
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = pbcopy.stdin.unwrap();
    while let Some(chunk) = stream.next().await {
        stdin.write_all(&chunk.unwrap()).unwrap();
    }
}

#[tokio::main]
async fn serve(host: String, port: u16) {
    println!("Listening on {}:{}", host, port);

    let addr = [host, port.to_string()].join(":");
    let listener = TcpListener::bind(addr).await.unwrap();

    let app = Router::new()
        .route("/", get(pbpaste))
        .route("/", post(pbcopy));
    axum::serve(listener, app).await.unwrap();
}

fn main() {
    let args = Args::parse();
    if args.daemon {
        let daemonize = Daemonize::new().working_directory("/");
        match daemonize.start() {
            Ok(_) => serve(args.host, args.port),
            Err(e) => eprintln!("Error: {}", e),
        }
    } else {
        serve(args.host, args.port)
    }
}
