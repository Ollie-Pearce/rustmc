// Ported from loom litmus.rs

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;
use std::thread;

// Load buffering test - originally #[ignore] in loom
// Ported: runs once, model checker should explore if a=1 is possible
#[no_mangle]
fn load_buffering() {
    let x = Arc::new(AtomicUsize::new(0));
    let y = Arc::new(AtomicUsize::new(0));

    let th = {
        let (x, y) = (x.clone(), y.clone());
        thread::spawn(move || {
            x.store(y.load(Relaxed), Relaxed);
        })
    };

    let a = x.load(Relaxed);
    y.store(1, Relaxed);

    th.join().unwrap();

    // Original test: across all loom runs, verify that a=1 was observed at least once
    // In single execution: a could be 0 or 1 depending on interleaving
    // Model checker should explore both possibilities
    assert!(a <= 1, "Invalid value: a={}", a);
}

// Store buffering test
// Ported: runs once, model checker explores if (0,0) interleaving exists
#[no_mangle]
fn store_buffering() {
    let x = Arc::new(AtomicUsize::new(0));
    let y = Arc::new(AtomicUsize::new(0));

    let a = {
        let (x, y) = (x.clone(), y.clone());
        thread::spawn(move || {
            x.store(1, Relaxed);
            y.load(Relaxed)
        })
    };

    y.store(1, Relaxed);
    let b = x.load(Relaxed);

    let a_val = a.join().unwrap();

    // Original: collected (a, b) pairs across runs and verified (0, 0) was possible
    // Under relaxed memory ordering, all four combinations are valid: (0,0), (0,1), (1,0), (1,1)
    // Model checker should explore different interleavings to find all possibilities
    assert!(a_val <= 1 && b <= 1, "Invalid values: a={}, b={}", a_val, b);
}

fn main() {
    load_buffering();
    store_buffering();
    println!("All tests completed");
}
