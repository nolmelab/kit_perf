use tokio::sync::oneshot;
use tokio::task;

async fn main_task(tx: oneshot::Sender<()>) {
    println!("Main task started.");

    // Simulate some work
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    println!("Main task finished.");
    let _ = tx.send(());
}

async fn dependent_task(rx: oneshot::Receiver<()>) {
    println!("Dependent task started.");

    // Wait for the main task to finish
    rx.await.expect("Failed to receive message from main task.");

    println!("Dependent task woken up by the main task.");
}

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    let main_task_handle = tokio::spawn(main_task(tx));
    let dependent_task_handle = tokio::spawn(dependent_task(rx));

    // Wait for both tasks to complete
    tokio::try_join!(main_task_handle, dependent_task_handle)
        .expect("Failed to join tasks.");
}