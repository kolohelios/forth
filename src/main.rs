use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

mod map;

fn main() {
    let (clock_tx, clock_rx) = mpsc::channel();

    let handle = thread::spawn(move || clock(clock_tx));
    thread::spawn(move || display_clock(clock_rx));

    handle.join().unwrap();
}

fn clock(tx: mpsc::Sender<String>) {
    let now = Instant::now();
    for i in 1..10 {
        let message = format!("time is {}", now.elapsed().as_millis());
        tx.send(message).unwrap();
        thread::sleep(Duration::from_secs(5));
    }
}

fn display_clock(rx: mpsc::Receiver<String>) {
    for received in rx {
        println!("Got: {}", received);
    }
}
