use std::io::{self, Read};

use clap::Parser;
use reqwest;

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(short = 'H', long, default_value = "host.docker.internal")]
    host: String,

    #[arg(short, long, default_value_t = 12345)]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let url = format!("http://{}:{}", args.host, args.port);

    let mut content = Vec::new();
    io::stdin().read_to_end(&mut content).unwrap();

    let client = reqwest::Client::new();
    match client.post(url).body(content).send().await {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}
