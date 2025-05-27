use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from spawned thread.");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap(); // will finish the spawn thread loop before starting the main one.

    for i in 1..5 {
        println!("hi number {i} from main thread.");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // moving data one thread to another

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || println!("Here is a vector :{v:?}"));

    // drop(v);

    handle.join().unwrap();

    // message passing to transfer data between threads

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // println!("val is {val}");
    });

    for received in rx {
        println!("Got: {received}");
    }
}
