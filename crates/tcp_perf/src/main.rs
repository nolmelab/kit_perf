use clap::Parser;

mod tcp_client;
mod tcp_server;
mod error;
mod stat;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
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

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let args = Args::parse();

    if args.mode == "server" {
        // Server를 만들고 실행한다.
        let mut server = tcp_server::TcpServer::new(args.listen).await?;
        server.run().await?;
        
    }

    if args.mode != "client" {
        println!("server or client mode is supported");
        // 에러를 리턴한다.
    }

    // Client를 만들고 실행한다.

    Ok(())
}
