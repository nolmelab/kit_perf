use std::sync::mpsc::{ self, Receiver, Sender };
use std::thread;
use tokio::runtime::Builder;
use anyhow;
use crate::event::Event;

pub fn run(args: super::Args) {
    let (tx, rx) = mpsc::channel::<Event>();

    let runtime = Builder::new_multi_thread().thread_name("run-tcp").build().unwrap();

    let args2 = args.clone();

    thread::spawn(move || {
        run_ui(&args2, rx);
    });

    let _result = runtime.block_on(run_tcp(&args, tx));
}

async fn run_tcp(args: &super::Args, tx: Sender<Event>) -> Result<(), anyhow::Error> {

    let mut num_connection = 0;

    while num_connection < args.conns {


    }

    Ok(())
}

fn run_ui(args: &super::Args, rx: Receiver<Event>) {}
