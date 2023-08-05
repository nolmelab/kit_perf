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
        
    }
}
