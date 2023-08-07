//! frame은 프로토콜이다. AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt와
//! 함께 살펴본다.

#[cfg(test)]
mod tests {
    use anyhow;
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt; // for write_all()
    use tokio::io::AsyncReadExt; 

    #[tokio::test]
    async fn tokio_stream() -> anyhow::Result<()> {
        let mut file = File::create("foo.txt").await?;
        file.write_all(b"hello, world!").await?;

        let mut fs = File::open("foo.txt").await?;
        let mut contents = vec![];
        fs.read_to_end(&mut contents).await?;

        Ok(())
    }

    #[test]
    fn understand_async_write_ext() {
        // AsyncWrite의 확장 (extension)이다. trait 상속으로 구현한 트레이트이다. 
        // write(&'a mut self, src: &'a [u8]) -> Write<'a, Self>
        // Write가 Future이다. Write의 poll()은 AsyncWrite::poll_write()를 호출한다. 
        // 많은 Future들이 있다. WriteAll, WriteAllBuf, Flush 등 
        // 그리고, 많은 유틸리티 함수들이 있다. 

        // AsyncWrite에 대해 Blanket 구현을 갖는다. 
        // impl<W: AsyncWrite + ?Sized> AsyncWriteExt for W {}
    }

    #[test]
    fn look_at_file_struct() {
        // tokio::fs::File
        // std::fs, std::task, std::pin 등을 사용한다. 
        // asyncfy() 함수는 spawn_blocking()으로 실행한다. 

        // 문서화가 얼마나 중요한지 러스트가 잘 보여준다. 

        // 사용에서 필요한 부분들: 
        // - OpenOptions 
        // - AsyncReadExt, AsyncWriteExt
        // - AsyncRead 구현         

        fn asyncread_impl() {
            // tokio::io::ReadBuf를 사용한다. 
            // buf: &'a mut [MaybeUninit<u8>]
            // filled, initialized : usize 필드들 
            // put_slice() 등을 갖고 가장 중요하게 bytes::BufMut를 unsafe하게 구현한다. 

            // 다른 언어에서 이만큼 이해하려고 한 적이 없다. 왜 그랬을까? 
            // 적당한 수준에서 만족했다. 그리고, 그런 것이 습관으로 남아있다. 
        }

        fn asyncseek_impl() {
            // AsyncSeek를 File이 구현하고 있다. 
            // AsyncWriteExt나 AsyncReadExt처럼 AsyncSeekExt가 Future를 
            // 반환하는 함수들을 갖고, await 구문을 사용할 수 있게 한다.  
            // 두 개 층으로 나누어 구현한다. 
        }

        asyncread_impl();
        asyncseek_impl();
    }

    #[tokio::test]
    async fn understand_tcp_stream() {
        use tokio::net::TcpStream;
        use tokio::io::BufWriter;

        async fn buf_writer() -> anyhow::Result<()> {
            // BufWriter는 W가 AsyncWrite일 경우 동작하도록 구현된다. 
            // 그리고, W가 AsyncWrite일 경우 AsyncWrite를 구현한다. 
            // 따라서, BufWriter는 AsyncWriteExt를 통해 Future로 사용할 수 있다. 

            // 아직 Pin이 불편하다. 뭔지는 아는 듯 한데 구현을 전부 알지 못한다.  

            let mut file = File::create("foo.txt").await?;
            file.write_all(b"hello, world!").await?;

            drop(file);

            let mut file = File::open("foo.txt").await?;
            let mut bw = BufWriter::new(file);
            bw.write_f32(0_f32).await?;

            Ok(())
        }

        let r = buf_writer().await;
    }

    #[tokio::test]
    async fn frame_plan() {
        // 이제는 frame에 대해 별로 걱정하지 않아도 되는 수준이 된 것 같다. 
        // TcpStream이 AsyncWrite, AsyncRead를 구현하고 이를 통해 
        // 읽고 쓸 수 있는 기능을 확장할 수 있게 하기 때문이다. 
        // 

    }

    #[tokio::test]
    async fn length_codec_impl() {
        // length를 갖는 코덱을 직접 구현한다. 
        // 
    }

}
