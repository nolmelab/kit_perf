#[cfg(test)]
mod tests {
    use anyhow;
    use tokio::net::{ TcpListener, TcpStream };
    use tokio::io::{ AsyncReadExt, AsyncWriteExt };
    use tokio_stream::StreamExt;
    use futures::SinkExt; // StreamExt는 tokio에 있는 걸 사용하고
    use tokio_util::codec::{ LengthDelimitedCodec, BytesCodec, Decoder };

    #[tokio::test]
    async fn understand_bytes_codec() {
        async fn server() -> anyhow::Result<()> {
            let listener = TcpListener::bind("127.0.0.1:7000").await?;

            loop {
                // Asynchronously wait for an inbound socket.
                let (stream, _) = listener.accept().await?;

                // 읽기만 해서는 안 되므로 쓰는 것도 필요하다.
                // Framed<S, C>::next()는 StreamExt의 함수라고 한다. Future를 돌려주는.
                // 마치 iterator처럼 사용할 수 있게 하므로 편리하다.

                // 쓰는 건 어떻게 하는가?

                tokio::spawn(async move {
                    // We're parsing each socket with the `BytesCodec` included in `tokio::codec`.
                    let mut framed = BytesCodec::new().framed(stream);

                    loop {
                        if let Some(message) = framed.next().await {
                            match message {
                                Ok(bytes) => {
                                    let _ = framed.send(bytes).await;
                                }
                                Err(err) => println!("Socket closed with error: {:?}", err),
                            }
                        }
                    }

                    // println!("Socket received FIN packet and closed connection");
                });
            }
        }

        async fn client() -> anyhow::Result<()> {
            let mut stream = TcpStream::connect("127.0.0.1:7000").await?;

            let mut buf = vec![1; 1024];

            let mut echo_count = 0;

            // In a loop, read data from the socket and write the data back.
            loop {
                stream.write_all(&buf[0..1024]).await.expect("failed to write data to socket");

                let n = stream.read(&mut buf).await.expect("failed to read data from socket");

                if n == 0 {
                    return anyhow::Ok(()); // anyhow에서 Ok 함수를 제공. 왜 필요하지?
                }

                echo_count += 1;
                println!("client: {}", echo_count);
            }
        }

        let task_1 = tokio::spawn(async {
            let _ = server().await;
        });

        let task_2 = tokio::spawn(async {
            let _ = client().await;
        });

        let _ = task_1.await;
        let _ = task_2.await;
    }

    #[test]
    fn read_bytes_codec() {
        fn bytes_codec() {
            // BytesCodec은 bytes를 패킷 포맷으로 한다. 가장 단순하다. 따라서, Codec과
            // Framed를 이해하기 좋다.

            // Turn an AsyncRead into a stream of Result<BytesMut, Error>
            // 러스트 커뮤니티의 문서화에서 가장 좋아하는 스타일이다. 간결하다.

            // 기막힌 덤이다. Cursor가 AsyncRead를 구현하고 있다. 그러면 패킷 처리 관련
            // 단위 테스트를 io::Cursor로 따로 다 할 수 있다.
            // async fn open(_name: &str) -> Result<impl AsyncRead, std::io::Error> {
            //     use std::io::Cursor;
            //     Ok(Cursor::new(vec![0, 1, 2, 3, 4, 5]))
            // }

            // fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<BytesMut>, io::Error> {
            // fn encode(&mut self, data: Bytes, buf: &mut BytesMut) -> Result<(), io::Error> {
            // fn encode(&mut self, data: BytesMut, buf: &mut BytesMut) -> Result<(), io::Error> {

            // encode를 Bytes, BytesMut에 대해 따로 구현한다.
            // let mut framed = BytesCodec::new().framed(stream);
            // 위 코드에서 framed() 함수는 Decoder의 함수이다.
            // BytesCodec은 Encoder와 Decoder를 모두 구현한다.

            //
        }

        fn framed() {
            // AsyncRead + AsyncWrite인 T에 대해 구현한다.
            // Framed는 Stream과 Sink를 구현한다.
            // Stream은 poll_next(), Sink는 전송을 위한 함수를 갖는다.
            // split를 해서 Stream과 Sink로 처리할 수 있다.

            // FramedImpl에 세부적인 IO 구현을 숨겨둔다.

            // Stream과 Sink로 분할하여 처리하고 Framed가 구현한 Stream, Sink에 대해
            // StreamExt와 SinkExt의 Future들로 둘러싼 함수들을 사용하면 편리하다.

            // 이와 같이 trait, Future 쌍의 처리가 tokio에 많이 있다.
            // 매우 강력한 추상화이고 매우 편리하다.
        }

        bytes_codec();
        framed();
    }
}
