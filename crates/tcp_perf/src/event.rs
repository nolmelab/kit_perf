
use std::net::SocketAddr;

/// channel을 통해 상태 리포팅을 위한 이벤트 
pub enum Event {
    /// 상태를 알리는 메세지
    State(String),
    /// 클라이언트에서 연결 했을 때
    Connected(SocketAddr),          
    /// 서버에서 연결을 받았을 때
    Accepted(SocketAddr, u32), 
    /// 연결이 종료되었을 때
    Closed,
    /// 에코 받았을 때 
    EchoRecv(f32), 
    /// 에러 발생시 설명과 함께 
    Error(String)
}