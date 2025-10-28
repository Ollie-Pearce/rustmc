// Ported from loom rwlock_regression1.rs
// Test for RwLock with two writers

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;

#[no_mangle]
fn rwlock_two_writers() {
    let lock = Arc::new(RwLock::new(1));
    let c_lock = lock.clone();
    let c_lock2 = lock;

    let atomic = Arc::new(AtomicUsize::new(0));
    let c_atomic = atomic.clone();
    let c_atomic2 = atomic;

    let t1 = thread::spawn(move || {
        let mut w = c_lock.write().unwrap();
        *w += 1;
        c_atomic.fetch_add(1, Ordering::Relaxed);
    });

    let t2 = thread::spawn(move || {
        let mut w = c_lock2.write().unwrap();
        *w += 1;
        c_atomic2.fetch_add(1, Ordering::Relaxed);
    });

    t1.join().unwrap();
    t2.join().unwrap();
}

fn main() {
    rwlock_two_writers();
    println!("Test completed");
}
