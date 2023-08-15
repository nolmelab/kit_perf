use std::sync::mpsc::{self, Receiver, Sender};
use tokio::runtime::Builder;
use crate::event::Event;
use crate::error::Error;


pub fn run(args: &super::Args) {

    let (tx, rx) = mpsc::channel::<Event>();

    let runtime = Builder::new_multi_thread()
        .thread_name("run-tcp")
        .build()
        .unwrap();

    let _result = runtime.block_on(run_tcp(tx));

    run_ui(rx);
}

async fn run_tcp(tx: Sender<Event>) -> Result<(), Error> {

    Ok(())
}

fn run_ui(rx: Receiver<Event>) {

}
