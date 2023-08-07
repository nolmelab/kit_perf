#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::io::SeekFrom;
    use std::io::prelude::*; // prelude::*를 use해야 seek 함수가 Cursor에 대해 활성화 됨
    use bytes::{Buf, BufMut, BytesMut};

    // std::io::Cursor를 Stream, Sink 관점에서 이해한다. 
    #[tokio::test]
    async fn understand_io_cursor() {
        let mut mbuf = BytesMut::with_capacity(1024);
        mbuf.put_f32(0.1_f32);

        // Cursor는 AsRef<[u8]>을 구현하면 Read, Write를 구현하고 Seek를 사용할 수 있게 한다.
        let mut buf = Cursor::new(&mbuf);

        // bytes::Buf가 io::Cursor에 대해 구현되어 있다. 
        // bytes::BufMut는 Cursor에 대해 구현되어 있지 않다. 
        // Write를 구현하므로 쓸 수도 있고, 예시도 있다. 

        // SeekFrom
        buf.set_position(0);
        // Buf impl for Cursor
        let fv = buf.get_f32();

        assert_eq!(fv, 0.1_f32);
    }
}