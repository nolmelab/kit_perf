//! TcpSession을 보관하고 관리한다.
//!

// 작업하는 동안
#![allow(unused)]

use std::ops::Deref;
use std::sync::{ Arc, atomic::AtomicU64 };
use std::net::SocketAddr;
use dashmap::DashMap;

use tokio::{ sync::RwLock, io::AsyncReadExt, io::AsyncWriteExt };
use tokio::net::TcpStream;
use tokio::sync::mpsc::{ UnboundedReceiver, UnboundedSender };
use bytes::{ BytesMut, BufMut, Buf, Bytes };

use super::error::Error;

// 하위의 통신 처리 클래스. TcpStream에 대한 소유권을 갖는다.
pub struct Session {
    id: u64,
    stream: TcpStream,
    addr: SocketAddr,
    buf: BytesMut,
    receiver: UnboundedReceiver<Message>,
}

pub struct Peer {
    id: u64,
    sender: UnboundedSender<Message>,
}

pub enum Message {
    Close,
    Send(Bytes),
}

// 상위 노드. 피어를 다룬다.
pub struct TcpNode {
    peers: DashMap<u64, Peer>,
    next_id: AtomicU64,
}

impl TcpNode {
    pub fn new() -> TcpNode {
        Self {
            peers: DashMap::new(),
            next_id: AtomicU64::new(1),
        }
    }

    pub fn start(&self, stream: TcpStream) -> Option<u64> {
        let addr = stream.peer_addr();

        if let Ok(addr) = addr {
            let id = self.next_id.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Message>();
            let mut session = Session::new(id, stream, addr, rx);

            // 세션 관리는 tokio task에서만 한다. tokio에서 실행하는 것을 관리하려고 하면
            // 소유권 관리가 매우 애매해지고 락 처리도 어려워진다. 
            // 자세히 한번 정리할 필요가 있다. 지금은 채널을 통해 상위 인터페이스를 분리한다.
            // 버퍼만 락으로 분리해서 Arc로 공유하는 방법도 있다. 이것이 우리가 C++에서 
            // 사용해온 방식이다. 
            tokio::spawn(async move {
                session.run();
            });

            // Peer들을 상위에서 사용한다. 수신 처리는 좀 더 나중에 고민한다. 
            let peer = Peer::new(&id, tx);
            self.peers.insert(id, peer);

            return Some(id);
        }
        None
    }

    pub fn close(&mut self, id: u64) {
        // Peer를 찾아서, close 명령을 보낸다.
    }
}

// 상위 노드를 통해 전송한다. 수신 처리는 여러 Fn들로 해야 한다.
impl Session {
    fn new(
        id: u64,
        stream: TcpStream,
        addr: SocketAddr,
        receiver: UnboundedReceiver<Message>
    ) -> Self {
        let session = Session {
            id,
            stream: stream,
            addr,
            buf: BytesMut::new(),
            receiver,
        };

        session
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        // read and write

        Ok(())
    }

    // send는 원래 락이 필요하다.
}

impl Peer {
    fn new(id: &u64, sender: UnboundedSender<Message>) -> Self {
        Self {
            id: *id,
            sender,
        }
    }
}
