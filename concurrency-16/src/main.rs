use std::rc::Rc;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // handle.join().unwrap();
    let v = vec![1, 2, 3];

    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // drop(v);

    handle2.join().unwrap();

    let (tx, rx) = mpsc::channel();
    // multiple producer, single consumer

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val); // that is not valid as the value lost ownership
        // spawned thread needs to own the transmitter
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    // also try_recv for trying and not blocking the thread

    // sending multiple values
    let (tx2, rx2) = mpsc::channel();

    let tx3 = tx2.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx3.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx2 {
        println!("Got: {}", received);
    }

    // mutex = mutual exclusion - only one thread can access the data
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
    // mutex is a smart pointer

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
    // using Arc = atomic reference coun9ter type
    // thread safety has performance penalty
}
