// Ported from loom thread_api.rs

use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;

#[no_mangle]
fn initial_thread() {
    thread::current().id(); // can call id()
    assert_eq!(None, thread::current().name());
}

#[no_mangle]
fn many_joins() {
    let mut handles = vec![];
    let mutex = Arc::new(Mutex::new(()));
    let lock = mutex.lock().unwrap();

    for _ in 1..3 {
        let mutex = mutex.clone();
        handles.push(thread::spawn(move || {
            mutex.lock().unwrap();
        }));
    }

    std::mem::drop(lock);

    for handle in handles.into_iter() {
        let _ = handle.join();
    }
}

#[no_mangle]
fn alt_join() {
    let arcmut: Arc<Mutex<Option<thread::JoinHandle<()>>>> = Arc::new(Mutex::new(None));
    let lock = arcmut.lock().unwrap();

    let arcmut2 = arcmut.clone();

    let th1 = thread::spawn(|| {});
    let th2 = thread::spawn(move || {
        arcmut2.lock().unwrap();
        let _ = th1.join();
    });
    let th3 = thread::spawn(move || {});
    std::mem::drop(lock);
    let _ = th3.join();
    let _ = th2.join();
}

#[no_mangle]
fn threads_have_unique_ids() {
    let (tx, rx) = channel();
    let th1 = thread::spawn(move || tx.send(thread::current().id()));
    let thread_id_1 = rx.recv().unwrap();

    assert_eq!(th1.thread().id(), thread_id_1);
    assert_ne!(thread::current().id(), thread_id_1);
    let _ = th1.join();

    let (tx, rx) = channel();
    let th2 = thread::spawn(move || tx.send(thread::current().id()));
    let thread_id_2 = rx.recv().unwrap();
    assert_eq!(th2.thread().id(), thread_id_2);
    assert_ne!(thread::current().id(), thread_id_2);
    assert_ne!(thread_id_1, thread_id_2);
    let _ = th2.join();
}

#[no_mangle]
fn thread_names() {
    let (tx, rx) = channel();
    let th = thread::spawn(move || tx.send(thread::current().name().map(|s| s.to_string())));
    assert_eq!(None, rx.recv().unwrap());
    assert_eq!(None, th.thread().name());
    let _ = th.join();

    let (tx, rx) = channel();
    let th = thread::Builder::new()
        .spawn(move || tx.send(thread::current().name().map(|s| s.to_string())))
        .unwrap();
    assert_eq!(None, rx.recv().unwrap());
    assert_eq!(None, th.thread().name());
    let _ = th.join();

    let (tx, rx) = channel();
    let th = thread::Builder::new()
        .name("foobar".to_string())
        .spawn(move || tx.send(thread::current().name().map(|s| s.to_string())))
        .unwrap();
    assert_eq!(Some("foobar".to_string()), rx.recv().unwrap());
    assert_eq!(Some("foobar"), th.thread().name());

    let _ = th.join();
}

#[no_mangle]
fn thread_stack_size() {
    const STACK_SIZE: usize = 1 << 12; // Original: 1 << 16 (1 << 12 takes 40s)
    let body = || {
        // Allocate a large array on the stack.
        std::hint::black_box(&mut [0usize; STACK_SIZE]);
    };
    thread::Builder::new()
        .stack_size(
            // Include space for function calls in addition to the array.
            2 * STACK_SIZE,
        )
        .spawn(body)
        .unwrap()
        .join()
        .unwrap()
}

#[no_mangle]
fn park_unpark() {
    thread::current().unpark();
    thread::park();
}

fn main() {
    initial_thread();
    many_joins();
    alt_join();
    threads_have_unique_ids();
    thread_names();
    thread_stack_size();
    park_unpark();
    println!("All tests completed");
}
