use crossbeam::channel::Receiver;
use tokio::sync::mpsc;

struct Message {
    payload: String,
}

impl Message {
    fn new(m: &str) -> Self {
        Self {
            payload: m.to_string()
        }
    }
}

async fn receiver(rx: &mut mpsc::Receiver<Message>) {

} 

async fn send(tx: &mpsc::Sender<Message>) {

} 

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(1024); 

    let tx2 = tx.clone();

    println!("start.");

    let j1 = tokio::spawn(async move {
        for _ in 0..10000000 {
            let _r = tx2.send(Message::new("hello")).await;
        }        
    });

    let j2 = tokio::spawn(async move {
        for _ in 0..10000000 {
            let _r = rx.recv().await; 
        }
    });

    let _ = j1.await;
    let _ = j2.await;

    println!("end.");
}