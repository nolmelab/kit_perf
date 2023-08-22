use crate::event::Event;
use anyhow;
use bytes::BytesMut;
use ratatui::{backend::CrosstermBackend, widgets::Paragraph, Terminal};
use std::sync::mpsc::{self, Receiver, Sender};
use std::{
    io::stdout,
    thread,
    time::Duration,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Builder;

pub fn run(args: super::Args) {
    let (tx, rx) = mpsc::channel::<Event>();

    let runtime = Builder::new_multi_thread()
        .enable_io()
        .thread_name("run-tcp")
        .build()
        .unwrap();

    // 소유권을 자세히 잘 정리하면 안정성에 큰 도움이 된다. 그것이 러스트다.
    let args2 = args.clone();

    // we need to spawn a thread for the ui since block_on() blocks on the current thread.
    thread::spawn(move || {
        let _ = run_ui(&args2, rx);
    });

    let _result = runtime.block_on(run_tcp(&args, tx));
}

async fn run_tcp(args: &super::Args, tx: Sender<Event>) -> Result<(), anyhow::Error> {
    // listener를 만들고 accept를 하면 task로 각 클라 연결에 대해 echo 처리
    let listener = TcpListener::bind(&args.listen).await?;

    let running = true;
    let mut count = 1;

    while running {
        let (stream, remote_addr) = listener.accept().await?;
        let evt = Event::Accepted(remote_addr, count);
        let _ = tx.send(evt);
        let echo_size = args.size.clone();

        tokio::spawn(async move {
            let _ = run_stream(echo_size, stream).await;
        });

        count += 1;
    }

    Ok(())
}

// write_buf()가 mut 참조를 필요로 한다. stream을 mut로 전달한다. 
async fn run_stream(echo_size: u32, mut stream: TcpStream) -> Result<(), anyhow::Error> {
    let mut buf = BytesMut::with_capacity(echo_size as usize);
    let run = true;

    while run {
        stream.read_buf(&mut buf).await?;
        stream.write_buf(&mut buf).await?;
        buf.clear(); 
    }

    Ok(())
}

fn run_ui(args: &super::Args, rx: Receiver<Event>) -> Result<(), anyhow::Error> {
    let stdout = stdout();
    // execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);

    // Terminal은 buffers: [Buffer; 2]를 갖는다. 이중 버퍼링으로 보인다.
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let mut running = true;
    let mut message = "waiting...".to_string();
    while running {
        terminal.draw(|f| f.render_widget(Paragraph::new(message.as_str()), f.size()))?;

        thread::sleep(Duration::from_millis(10));

        let ev = rx.recv()?;
        match ev {
            Event::Accepted(addr, count) => {
                message = format!("accepted. Count: {}, {:?}:{}", count, addr.ip(), addr.port());
            }
            _ => {}
        }
    }

    terminal.show_cursor()?;
    terminal.flush()?;
    Ok(())
}
