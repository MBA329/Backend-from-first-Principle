use std::sync::mpsc;
use std::thread;

fn counter_service(rx: mpsc::Receiver<i32>, done: mpsc::Sender<i32>) {
    let mut counter = 0;
    for delta in rx {
        counter += delta; // Only this thread modifies counter
    }
    done.send(counter).unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let (done_tx, done_rx) = mpsc::channel();

    thread::spawn(move || {
        counter_service(rx, done_tx);
    });

    for _ in 0..1000 {
        tx.send(1).unwrap();
    }
    drop(tx); // Close the channel

    let result = done_rx.recv().unwrap();
    println!("{}", result); // 1000, no race condition, no mutex needed
}
