// Ported from loom mutex.rs

use std::cell::UnsafeCell;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::{Arc, Mutex};
use std::thread;

#[no_mangle]
fn mutex_into_inner() {
    let lock = Arc::new(Mutex::new(0));

    let ths: Vec<_> = (0..2)
        .map(|_| {
            let lock = lock.clone();

            thread::spawn(move || {
                *lock.lock().unwrap() += 1;
            })
        })
        .collect();

    for th in ths {
        th.join().unwrap();
    }

    let lock = Arc::try_unwrap(lock).unwrap().into_inner().unwrap();
    assert_eq!(lock, 2);
}

fn main() {
    mutex_into_inner();
    println!("All tests completed");
}
