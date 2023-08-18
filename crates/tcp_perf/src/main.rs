use clap::Parser;

mod server;
mod client;
mod event;

/// Simple program to greet a person
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// execution mode - server or client
    #[arg(short, long)]
    mode: String,

    /// address to listen 
    #[arg(short, long, default_value_t = String::from("0.0.0.0:7001"))]
    listen: String,

    /// address to connect
    #[arg(short, long, default_value_t = String::from("0.0.0.0:7001"))]
    remote: String,
 
    /// Number of connections 
    #[arg(short, long, default_value_t = 1)]
    conns: u32,
}

fn main() {
    let args = Args::parse();

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
