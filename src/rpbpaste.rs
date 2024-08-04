use clap::Parser;

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

    match reqwest::get(url).await {
        Ok(response) => {
            let body = response.text().await.unwrap();
            print!("{}", body);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
