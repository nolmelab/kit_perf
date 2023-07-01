use std::future::{ Future, IntoFuture };
use std::pin::Pin;
use std::task::{ Context, Poll, Waker };
use std::sync::{ Arc, Mutex };
use tokio::task;

#[derive(Debug, Clone)]
struct MyFuture {
    waker: Arc<Mutex<Option<Waker>>>,
}

impl MyFuture {
    fn new() -> Self {
        Self {
            waker: Arc::new(Mutex::new(None)),
        }
    }

    fn wake_up(&self) {
        let waker = self.waker.lock().unwrap().take();
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}

impl Future for MyFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        *self.waker.lock().unwrap() = Some(cx.waker().clone());
        println!("I wake up!");
        Poll::Pending
    }
}

async fn main_task(future: Arc<Mutex<MyFuture>>) {
    println!("Main task started.");

    // Simulate some work
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    println!("Main task finished.");

    {
        println!("Waking up...");
        let future = future.lock().unwrap();
        future.wake_up();
    }

    println!("waiting...");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Arc contains immutable refrence. It just blocks dropping.
    let f1 = Arc::new(Mutex::new(MyFuture::new()));

    let f2 = Arc::clone(&f1);

    let main_task_handle = tokio::spawn(main_task(f2));

    {
        let mut future = f1.lock().unwrap();

        let f3 = &mut *future;

        let pinned_future = Pin::new(f3);

        let _result = pinned_future.await;
    }

    main_task_handle.await?;

    Ok(())
}

// oneshot channel을 사용하는 방법이 가장 깔끔하고 단순하다.
//
