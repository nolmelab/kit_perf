use tokio::net::TcpListener;

use super::error;
use super::stat;
use super::tcp_node::TcpNode;

pub struct TcpServer {
    listen_addr: String,
    listener: TcpListener,
    node: TcpNode,
}

impl TcpServer {
    pub async fn new(listen_addr: String) -> Result<Self, error::Error> {
        let listener = TcpListener::bind(&listen_addr).await?;

        let server = Self {
            listen_addr,
            listener,
            node : TcpNode::new()
        };

        Ok(server)
    }

    pub async fn run(&mut self) -> Result<(), error::Error> {
        loop {
            let stream = self.listener.accept().await?;
            // spawn task to handle the new stream

            // check input and exit when required to quit
        }
    }
}
