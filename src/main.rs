use clap::Parser;

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

fn main() {
    let args = Args::parse();

    println!(
        "daemon: {:?}, host: {:?}, port: {:?}",
        args.daemon, args.host, args.port
    );
}
