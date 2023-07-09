//! TcpSession을 보관하고 관리한다.
//!

// 작업하는 동안
#![allow(unused)]

use std::ops::Deref;
use std::sync::{ Arc, atomic::AtomicUsize };
use std::net::SocketAddr;
use dashmap::DashMap;

use tokio::{ sync::RwLock, io::AsyncReadExt, io::AsyncWriteExt };
use tokio::net::TcpStream;
use bytes::{ BytesMut, BufMut, Buf, Bytes };

use super::error::Error;

pub struct SharedSession(Arc<Session>);

pub struct TcpNode {
    sessions: DashMap<usize, SharedSession>,
    next_id: AtomicUsize,
}

pub struct Session {
    id: usize,
    stream: RwLock<TcpStream>,
    addr: SocketAddr,
    buf: BytesMut,
}

impl SharedSession {
    fn new(id: usize, stream: TcpStream, addr: SocketAddr) -> Self {
        let session = Session {
            id,
            stream: RwLock::new(stream),
            addr,
            buf: BytesMut::new(),
        };

        Self {
            0: Arc::new(session),
        }
    }

    // recv 처리를 한번만 한다.
    pub async fn recv(&self) -> Result<(), Error> {
        let result = self.0.recv().await;
        result
    }

    // send는 원래 락이 필요하다.
}

impl Clone for SharedSession {
    fn clone(&self) -> Self {
        SharedSession {
            0: self.0.clone(),
        }
    }
}

impl Session {
    async fn recv(&self) -> Result<(), Error> {
        let mut bytes = BytesMut::with_capacity(1024);

        // AsyncReadExt가 &mut 리시버를 필요로 한다. 안 좋다.
        let mut guard = self.stream.write().await;
        let result = guard.read(bytes.as_mut()).await;

        Ok(())
    }
}

impl TcpNode {
    pub fn new() -> TcpNode {
        Self {
            sessions: DashMap::new(),
            next_id: AtomicUsize::new(1)
        }
    }

    pub fn add(&self, stream: TcpStream) -> Option<SharedSession> {
        let addr = stream.peer_addr();
        if let Ok(addr) = addr {
            let s = SharedSession::new(
                self.next_id.fetch_add(1, std::sync::atomic::Ordering::AcqRel),
                stream,
                addr
            );

            let ns = s.clone();
            self.sessions.insert(s.0.id, ns);

            // start  

            return Some(s);
        }
        None
    }
}
