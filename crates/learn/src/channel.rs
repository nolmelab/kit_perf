use tokio::sync::mpsc;
use std::time::Instant;
use bytes::Bytes;

struct Message {
    payload: Box<Bytes>
}

impl Message {
    fn new(m: Box<Bytes>) -> Self {
        Self {
            payload: m
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(1024); 

    let tx2 = tx.clone();

    println!("start.");

    let now = Instant::now();

    let data = Box::new(Bytes::from_static(&[0_u8;1024][..]));

    let j1 = tokio::spawn(async move {
        for _ in 0..10000000 {
            let _r = tx2.send(Message::new(data.clone())).await;
        }        
    });

    let j2 = tokio::spawn(async move {
        for _ in 0..10000000 {
            let _r = rx.recv().await; 
        }
    });

    let _ = j1.await;
    let _ = j2.await;

    println!("end. elapsed: {:?}", now.elapsed());

    // i7 윈도우 PC에서 초당 5백만개를 처리할 수 있다. 코어는 3개 정도를 100% 가까이 
    // 사용한다. 2 개는 송수신이고 하나는 스케줄링 관련된 것으로 보인다. 
}