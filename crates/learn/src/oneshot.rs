use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();    
   
    tokio::spawn(async move {
        if let Err(_) = tx.send(3) {
            println!("the receiver dropped");
        }
    });

    match rx.await {
        Ok(v) => println!("got = {:?}", v),
        Err(_) => println!("the sender dropped"),
    } 
}


// Sender의 내부이다. 
// pub struct Sender<T> {
//    inner: Option<Arc<Inner<T>>>,
// ...
// }

// Receiver의 내부이다.
// pub struct Receiver<T> {
//    inner: Option<Arc<Inner<T>>>,
// ...
// }


//struct Inner<T> {
//    state: AtomicUsize,
//    value: UnsafeCell<Option<T>>,
//    tx_task: Task,
//    rx_task: Task,
//}

// UnsafeCell<Option<T>>를 갖는다. interior mutability는 주의깊게 잘 사용하면 된다. 
// tx_task, rx_task를 갖고 있다. 어디에 쓰는가?

// struct Task(UnsafeCell<MaybeUninit<Waker>>);
// Task가 위와 같이 초기화가 안 될 수 있는 Waker를 갖고 있다. 
// 

// Sender<T>::send() 
// - Inner<T>의 value를 지정하고 완료 처리 

// Receiver<T>는 Future이다. 
// poll()에서 Inner<T>를 통해 값을 받는다. 
//  

// Inner<T>는 Poll을 리턴하는 poll_recv()를 갖는다. 
// Poll을 통해 거의 Future로 만들어 사용한다. 

// 기본 아이디어와 구조는 위와 같다. Waker를 사용하여 다른 Future를 깨우고 
// Inner<T>에서 UnsafeCell을 통해 interior mutability를 갖는다. 
// AtomicUsize와 같은 것으로 Thread-safe하게 서로 알려주고 
// Poll 등의 수단을 통해 Receiver<T>를 Future로 만든다. 
// 



#[cfg(test)]
mod tests {

}