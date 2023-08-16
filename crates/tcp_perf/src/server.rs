use std::sync::mpsc::{ self, Receiver, Sender };
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Builder;
use anyhow;
use ratatui::{backend::CrosstermBackend, widgets::Paragraph, Terminal};
use std::{
    thread,
    io::stdout,
    time::{Duration, Instant},
};
use crate::event::Event;

pub fn run(args: super::Args) {
    let (tx, rx) = mpsc::channel::<Event>();

    let runtime = Builder::new_multi_thread().enable_io().thread_name("run-tcp").build().unwrap();

    // 소유권을 자세히 잘 정리하면 안정성에 큰 도움이 된다. 그것이 러스트다. 
    let args2 = args.clone();

    // we need to spawn a thread for the ui since block_on() blocks on the current thread.
    thread::spawn(move || {
        run_ui(&args2, rx);
    });

    let _result = runtime.block_on(run_tcp(&args, tx));
}

async fn run_tcp(args: &super::Args, tx: Sender<Event>) -> Result<(), anyhow::Error> {
    // listener를 만들고 accept를 하면 task로 각 클라 연결에 대해 echo 처리
    let listener = TcpListener::bind(&args.listen).await?;

    let running = true;

    while running {
        let (stream , remote_addr) = listener.accept().await?;

        let evt = Event::Accepted(remote_addr);
        let _ = tx.send(evt);

        tokio::spawn(async move {
            let _ = run_stream(stream).await;
        });
    }

    Ok(())
}

async fn run_stream(stream: TcpStream) -> Result<(), anyhow::Error> {
    // recv and then send


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
            Event::Accepted(addr) => {
                message = format!("{:?}:{}", addr.ip(), addr.port());
            },
            _ => {

            }
        }
    }

    terminal.show_cursor()?;
    terminal.flush()?;
    Ok(())
}
