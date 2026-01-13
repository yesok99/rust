use std::thread;
use std::time::Duration;
use std::sync::mpsc;
fn main() {
    // let handle = thread::spawn(|| {
    //     for i in 0..5 {
    //         println!("spawned thread print {}", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 0..3 {
    //     println!("main thread print {}", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // handle.join().unwrap();
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        thread::sleep(Duration::from_secs(5));
        tx.send(val).unwrap();
    });

    println!("wait for 5 seconds...");
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}